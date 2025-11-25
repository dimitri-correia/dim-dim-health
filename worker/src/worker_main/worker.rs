use entities::{Job, JobEmail, TaskType};
use redis::{AsyncCommands, aio::ConnectionManager};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};

use crate::{
    mail_jobs::common_mail_jobs::handle_mail_job,
    worker_main::{
        env_loader::Settings,
        state::{self, WorkerState},
        telemetry,
    },
};

pub async fn worker_main() {
    let settings = match Settings::load_config() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize telemetry with OpenObserve if configured
    if let Err(e) = telemetry::init_telemetry(
        "dimdim-health-worker",
        settings.openobserve_endpoint.as_deref(),
        &settings.env_filter,
    ) {
        eprintln!("Failed to initialize telemetry: {}", e);
        std::process::exit(1);
    }

    info!(
        num_workers = settings.number_workers,
        "Starting DimDim Health Worker"
    );

    let worker_state = match state::WorkerState::create_from_settings(&settings).await {
        Ok(s) => {
            info!("Worker state initialized successfully");
            s
        }
        Err(e) => {
            error!(error = %e, "Failed to create worker state");
            std::process::exit(1);
        }
    };

    // Shared flag to indicate shutdown
    let shutdown = Arc::new(RwLock::new(false));

    // Spawn multiple worker tasks
    let mut handles = vec![];
    for i in 0..settings.number_workers {
        let worker_id = format!("worker-{i}");
        let worker_state = worker_state.clone();
        let shutdown = shutdown.clone();

        info!(worker_id = %worker_id, "Spawning worker task");
        let handle =
            tokio::spawn(
                async move { worker_loop(worker_state.clone(), worker_id, shutdown).await },
            );
        handles.push(handle);
    }

    info!(
        num_workers = settings.number_workers,
        "All worker tasks spawned and running"
    );

    // Wait for shutdown signal
    shutdown_signal().await;

    // Set shutdown flag
    {
        let mut shutdown_flag = shutdown.write().await;
        *shutdown_flag = true;
    }

    warn!("Shutdown signal received, waiting for workers to finish processing current jobs...");

    // Wait for all workers to complete
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(_) => debug!(worker_index = i, "Worker task completed"),
            Err(e) => error!(worker_index = i, error = %e, "Worker task panicked"),
        }
    }

    info!("All workers have shut down gracefully");

    // Shutdown telemetry gracefully
    telemetry::shutdown_telemetry();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            warn!("Received Ctrl+C signal");
        },
        _ = terminate => {
            warn!("Received SIGTERM signal");
        },
    }
}

#[instrument(skip(worker_state, shutdown), fields(worker_id = %worker_id))]
async fn worker_loop(worker_state: WorkerState, worker_id: String, shutdown: Arc<RwLock<bool>>) {
    info!("Worker started and listening for jobs");

    let mut redis = worker_state.redis.clone();
    let mut consecutive_errors = 0u32;

    loop {
        // Check if shutdown was requested
        {
            let should_shutdown = *shutdown.read().await;
            if should_shutdown {
                info!("Shutdown requested, stopping worker loop");
                break;
            }
        }

        match fetch_job(&mut redis).await {
            Ok(Some(job)) => {
                consecutive_errors = 0;
                debug!(task_type = %job.task_type, "Processing job");

                if let Err(err) = process(worker_state.clone(), job, &worker_id).await {
                    error!(
                        error = %err,
                        "Job processing failed"
                    );
                }
            }
            Ok(None) => {
                // Timeout, continue
                consecutive_errors = 0;
            }
            Err(e) => {
                consecutive_errors = consecutive_errors.saturating_add(1);
                error!(
                    error = %e,
                    consecutive_errors = consecutive_errors,
                    "Error fetching job from queue"
                );

                // Exponential backoff on consecutive errors (capped at 30 seconds)
                // Using checked_shl to avoid overflow, capping shift at 4 (2^4 = 16, then min with 30)
                let shift_amount = std::cmp::min(consecutive_errors, 4);
                let delay_secs = 1u64
                    .checked_shl(shift_amount)
                    .map(|v| std::cmp::min(v, 30))
                    .unwrap_or(30);
                let delay = Duration::from_secs(delay_secs);
                debug!(delay_secs = delay_secs, "Backing off before retry");
                tokio::time::sleep(delay).await;
            }
        }
    }

    info!("Worker stopped");
}

async fn fetch_job(redis: &mut ConnectionManager) -> Result<Option<Job>, redis::RedisError> {
    // BLPOP blocks until a job is available or timeout (5 seconds)
    let result: Option<(String, String)> = redis.blpop("jobs", 5.0).await?;

    match result {
        Some((_queue, job_data)) => {
            let job: Job = serde_json::from_str(&job_data).unwrap();
            Ok(Some(job))
        }
        None => Ok(None), // Timeout
    }
}

#[instrument(skip(worker_state, job), fields(worker_id = %worker_id, task_type = ?job.task_type))]
async fn process(worker_state: WorkerState, job: Job, worker_id: &str) -> anyhow::Result<bool> {
    info!("Processing job");

    let job_result = match job.task_type {
        TaskType::Email => {
            let job_email: JobEmail = serde_json::from_value(job.data)?;
            debug!(email_type = ?job_email, "Handling email job");
            handle_mail_job(worker_state, job_email).await
        }
    };

    match &job_result {
        Ok(_) => info!("Job completed successfully"),
        Err(e) => error!(error = %e, "Job failed"),
    }

    job_result
}
