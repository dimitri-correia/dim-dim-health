use chrono::{Datelike, Timelike, Utc};
use entities::{
    email_preferences, users, EmailType, Job, JobEmail, JobEmailYearlyRecap, TaskType,
};
use redis::AsyncCommands;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::time::Duration;
use tracing::{error, info};

use crate::worker_main::state::WorkerState;

/// Scheduled job for yearly recap emails
/// Runs on January 1st at 09:00 AM
pub async fn process_yearly_recap_queue(worker_state: WorkerState) {
    info!("Starting yearly recap scheduler");
    
    loop {
        let now = Utc::now();
        
        // Check if it's January 1st and around 9 AM (within a 5-minute window)
        if now.month() == 1 && now.day() == 1 && now.hour() == 9 && now.minute() < 5 {
            info!("Triggering yearly recap emails");
            match enqueue_yearly_recap_emails(&worker_state).await {
                Ok(count) => {
                    info!("Enqueued {} yearly recap emails", count);
                }
                Err(e) => {
                    error!("Error enqueuing yearly recap emails: {}", e);
                }
            }
            // Sleep for 1 hour to avoid re-triggering
            tokio::time::sleep(Duration::from_secs(3600)).await;
        } else {
            // Check every 5 minutes
            tokio::time::sleep(Duration::from_secs(300)).await;
        }
    }
}

async fn enqueue_yearly_recap_emails(worker_state: &WorkerState) -> anyhow::Result<usize> {
    // Find all users who have opted in for yearly recap
    let users_with_prefs = email_preferences::Entity::find()
        .filter(email_preferences::Column::YearlyRecap.eq(true))
        .find_also_related(users::Entity)
        .all(&worker_state.db)
        .await?;
    
    let mut count = 0;
    
    for (_pref, user_opt) in users_with_prefs {
        if let Some(user) = user_opt {
            let job_email_yearly_recap = JobEmailYearlyRecap {
                email: user.email.clone(),
                username: user.username.clone(),
            };
            
            let job_email = JobEmail {
                email_type: EmailType::YearlyRecap,
                data: serde_json::to_value(job_email_yearly_recap)?,
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
                    info!("Enqueued yearly recap email for user: {}", user.username);
                    count += 1;
                }
                Err(e) => {
                    error!("Failed to enqueue job to Redis: {}", e);
                }
            }
        }
    }
    
    Ok(count)
}
