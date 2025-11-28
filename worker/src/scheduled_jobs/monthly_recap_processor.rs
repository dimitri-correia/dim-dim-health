use entities::{
    monthly_recap_queue, users, EmailType, Job, JobEmail, JobEmailMonthlyRecap, TaskType,
};
use redis::AsyncCommands;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::time::Duration;
use tracing::{error, info};

use crate::worker_main::state::WorkerState;

/// Process the monthly recap queue and enqueue email jobs to Redis
pub async fn process_monthly_recap_queue(worker_state: WorkerState) {
    info!("Starting monthly recap queue processor");
    
    loop {
        match process_pending_recaps(&worker_state).await {
            Ok(count) => {
                if count > 0 {
                    info!("Processed {} monthly recap jobs", count);
                }
            }
            Err(e) => {
                error!("Error processing monthly recap queue: {}", e);
            }
        }
        
        // Check every 5 minutes
        tokio::time::sleep(Duration::from_secs(300)).await;
    }
}

async fn process_pending_recaps(worker_state: &WorkerState) -> anyhow::Result<usize> {
    // Find all unprocessed monthly recap queue items
    let pending_items = monthly_recap_queue::Entity::find()
        .filter(monthly_recap_queue::Column::Processed.eq(false))
        .find_also_related(users::Entity)
        .all(&worker_state.db)
        .await?;
    
    let count = pending_items.len();
    
    for (queue_item, user_opt) in pending_items {
        if let Some(user) = user_opt {
            // Create and enqueue the job
            let job_email_monthly_recap = JobEmailMonthlyRecap {
                email: user.email.clone(),
                username: user.username.clone(),
            };
            
            let job_email = JobEmail {
                email_type: EmailType::MonthlyRecap,
                data: serde_json::to_value(job_email_monthly_recap)?,
            };
            
            let job = Job {
                task_type: TaskType::Email,
                data: serde_json::to_value(job_email)?,
            };
            
            // Push to Redis
            let mut redis = worker_state.redis.clone();
            match redis
                .rpush::<_, _, ()>("jobs", serde_json::to_string(&job)?)
                .await
            {
                Ok(_) => {
                    info!("Enqueued monthly recap email for user: {}", user.username);
                    
                    // Mark as processed
                    let mut queue_item_active: monthly_recap_queue::ActiveModel = queue_item.into();
                    queue_item_active.processed = Set(true);
                    queue_item_active.processed_at = Set(Some(chrono::Utc::now().into()));
                    
                    if let Err(e) = queue_item_active.update(&worker_state.db).await {
                        error!("Failed to mark queue item as processed: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to enqueue job to Redis: {}", e);
                }
            }
        } else {
            error!("User not found for queue item: {}", queue_item.id);
        }
    }
    
    Ok(count)
}
