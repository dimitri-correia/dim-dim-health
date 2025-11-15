use entities::{Job, JobEmail, TaskType};
use redis::{AsyncCommands, aio::ConnectionManager};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{
    mail_jobs::common_mail_jobs::handle_mail_job,
    worker_main::{
        env_loader::Settings,
        state::{self, WorkerState},
    },
};

pub async fn worker_main() {
    let settings = Settings::load_config().expect("Failed to load configuration");

    tracing_subscriber::fmt()
        .with_env_filter(&settings.env_filter)
        .init();

    info!("Starting Worker...");

    let worker_state = state::WorkerState::create_from_settings(&settings)
        .await
        .expect("Failed to create Worker State");

    // Shared flag to indicate shutdown
    let shutdown = Arc::new(RwLock::new(false));

    // Spawn multiple worker tasks
    let mut handles = vec![];
    for i in 0..settings.number_workers {
        let worker_id = format!("worker-{i}");
        let worker_state = worker_state.clone();
        let shutdown = shutdown.clone();

        let handle = tokio::spawn(async move {
            worker_loop(worker_state.clone(), worker_id, shutdown).await
        });
        handles.push(handle);
    }

    // Wait for shutdown signal
    shutdown_signal().await;

    // Set shutdown flag
    {
        let mut shutdown_flag = shutdown.write().await;
        *shutdown_flag = true;
    }

    info!("Shutdown signal received, waiting for workers to finish processing current jobs...");

    // Wait for all workers to complete
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Worker task panicked: {}", e);
        }
    }

    info!("All workers have shut down gracefully");
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
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received SIGTERM signal");
        },
    }
}

async fn worker_loop(worker_state: WorkerState, worker_id: String, shutdown: Arc<RwLock<bool>>) {
    info!("{}: Worker started", worker_id);

    let mut redis = worker_state.redis.clone();

    loop {
        // Check if shutdown was requested
        {
            let should_shutdown = *shutdown.read().await;
            if should_shutdown {
                info!("{}: Shutdown requested, stopping worker loop", worker_id);
                break;
            }
        }

        match fetch_job(&mut redis).await {
            Ok(Some(job)) => {
                if let Err(err) = process(worker_state.clone(), job, &worker_id).await {
                    error!("{worker_id}: Error processing with erorr {err}",);
                }
            }
            Ok(None) => {
                // Timeout, continue
            }
            Err(e) => {
                error!("{}: Error fetching job: {}", worker_id, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    info!("{}: Worker stopped", worker_id);
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

async fn process(worker_state: WorkerState, job: Job, worker_id: &str) -> anyhow::Result<bool> {
    info!("{}: Processing job: {}", worker_id, job);

    let job_result = match job.task_type {
        TaskType::Email => {
            let job_email: JobEmail = serde_json::from_value(job.data)?;
            handle_mail_job(worker_state, job_email).await
        }
    };

    info!("{}: Completed job: {}", worker_id, job.task_type);
    job_result
}
