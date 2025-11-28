use chrono::{Datelike, Timelike, Utc, Weekday};
use entities::{
    email_preferences, users, EmailType, Job, JobEmail, JobEmailWeeklyRecap, TaskType,
};
use redis::AsyncCommands;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::time::Duration;
use tracing::{error, info};

use crate::worker_main::state::WorkerState;

/// Scheduled job for weekly recap emails
/// Runs every Monday at 09:00 AM
pub async fn process_weekly_recap_queue(worker_state: WorkerState) {
    info!("Starting weekly recap scheduler");
    
    loop {
        let now = Utc::now();
        
        // Check if it's Monday and around 9 AM (within a 5-minute window)
        if now.weekday() == Weekday::Mon && now.hour() == 9 && now.minute() < 5 {
            info!("Triggering weekly recap emails");
            match enqueue_weekly_recap_emails(&worker_state).await {
                Ok(count) => {
                    info!("Enqueued {} weekly recap emails", count);
                }
                Err(e) => {
                    error!("Error enqueuing weekly recap emails: {}", e);
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

async fn enqueue_weekly_recap_emails(worker_state: &WorkerState) -> anyhow::Result<usize> {
    // Find all users who have opted in for weekly recap
    let users_with_prefs = email_preferences::Entity::find()
        .filter(email_preferences::Column::WeeklyRecap.eq(true))
        .find_also_related(users::Entity)
        .all(&worker_state.db)
        .await?;
    
    let mut count = 0;
    
    for (_pref, user_opt) in users_with_prefs {
        if let Some(user) = user_opt {
            let job_email_weekly_recap = JobEmailWeeklyRecap {
                email: user.email.clone(),
                username: user.username.clone(),
            };
            
            let job_email = JobEmail {
                email_type: EmailType::WeeklyRecap,
                data: serde_json::to_value(job_email_weekly_recap)?,
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
                    info!("Enqueued weekly recap email for user: {}", user.username);
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
