use entities::Job;
use redis::AsyncCommands;
use std::time::Duration;
use tracing::{error, info};

pub async fn worker_main() {
    tracing_subscriber::fmt::init();

    let worker_id = std::env::var("WORKER_ID").unwrap_or_else(|_| "worker-1".to_string());
    let num_workers = std::env::var("NUM_WORKERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);

    info!(
        "Starting {} with {} concurrent workers",
        worker_id, num_workers
    );

    server_main(worker_id, num_workers).await
}

pub async fn server_main(worker_id: String, num_workers: usize) {
    let client = redis::Client::open("redis://127.0.0.1:6379/").expect("redis not available");
    let manager = client
        .get_connection_manager()
        .await
        .expect("redis connection impossible");

    info!("Connected to Redis");

    // Spawn multiple worker tasks
    let mut handles = vec![];
    for i in 0..num_workers {
        let manager = manager.clone();
        let worker_id = format!("{}-{}", worker_id, i);

        let handle = tokio::spawn(async move { worker_loop(manager, worker_id).await });
        handles.push(handle);
    }

    // Wait for all workers
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Worker task panicked: {}", e);
        }
    }
}

async fn worker_loop(mut con: redis::aio::ConnectionManager, worker_id: String) {
    info!("{}: Worker started", worker_id);

    loop {
        match fetch_job(&mut con).await {
            Ok(Some(job)) => {
                // info!("{}: Got job {}", worker_id, job.id);
                if !process(job, &worker_id).await {
                    error!("{}: Error processing job: {}", worker_id, "todo");
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

async fn fetch_job(
    con: &mut redis::aio::ConnectionManager,
) -> Result<Option<Job>, redis::RedisError> {
    // BLPOP blocks until a job is available or timeout (5 seconds)
    let result: Option<(String, String)> = con.blpop("jobs", 5.0).await?;

    match result {
        Some((_queue, job_data)) => {
            let job: Job = serde_json::from_str(&job_data).unwrap();
            Ok(Some(job))
        }
        None => Ok(None), // Timeout
    }
}

async fn process(job: Job, worker_id: &str) -> bool {
    // info!("{}: Processing job: {}", worker_id, job);

    // Simulate work
    tokio::time::sleep(Duration::from_secs(2)).await;

    // info!("{}: Completed job: {}", worker_id, job);
    true
}
