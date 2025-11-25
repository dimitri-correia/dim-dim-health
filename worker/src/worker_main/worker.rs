use entities::{Job, JobEmail, TaskType};
use redis::{AsyncCommands, aio::ConnectionManager};
use std::time::Duration;
use tracing::{error, info};

use crate::{
    mail_jobs::common_mail_jobs::handle_mail_job,
    scheduled_jobs::monthly_recap_processor::process_monthly_recap_queue,
    scheduled_jobs::weekly_recap_processor::process_weekly_recap_queue,
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

    // Spawn multiple worker tasks
    let mut handles = vec![];
    
    // Spawn the monthly recap queue processor
    let monthly_processor_state = worker_state.clone();
    let monthly_processor_handle = tokio::spawn(async move {
        process_monthly_recap_queue(monthly_processor_state).await
    });
    handles.push(monthly_processor_handle);
    
    // Spawn the weekly recap queue processor
    let weekly_processor_state = worker_state.clone();
    let weekly_processor_handle = tokio::spawn(async move {
        process_weekly_recap_queue(weekly_processor_state).await
    });
    handles.push(weekly_processor_handle);
    
    for i in 0..settings.number_workers {
        let worker_id = format!("worker-{i}");

        let worker_state = worker_state.clone();

        let handle =
            tokio::spawn(async move { worker_loop(worker_state.clone(), worker_id).await });
        handles.push(handle);
    }

    // Wait for all workers
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Worker task panicked: {}", e);
        }
    }
}

async fn worker_loop(worker_state: WorkerState, worker_id: String) {
    info!("{}: Worker started", worker_id);

    let mut redis = worker_state.redis.clone();

    loop {
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
