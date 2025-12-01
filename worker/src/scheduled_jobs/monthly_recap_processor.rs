use chrono::{Datelike, Timelike, Utc};
use entities::{
    EmailType, Job, JobEmail, JobEmailMonthlyRecap, TaskType, email_preferences, users,
};
use redis::AsyncCommands;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::time::Duration;
use tracing::{error, info};

use crate::worker_main::state::WorkerState;

/// Scheduled job for monthly recap emails
/// Runs on the 1st of each month at 09:00 AM
pub async fn process_monthly_recap_queue(worker_state: WorkerState) {
    info!("Starting monthly recap scheduler");

    loop {
        let now = Utc::now();

        // Check if it's the 1st of the month and around 9 AM (within a 5-minute window)
        if now.day() == 1 && now.hour() == 9 && now.minute() < 5 {
            info!("Triggering monthly recap emails");
            match enqueue_monthly_recap_emails(&worker_state).await {
                Ok(count) => {
                    info!("Enqueued {} monthly recap emails", count);
                }
                Err(e) => {
                    error!("Error enqueuing monthly recap emails: {}", e);
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

async fn enqueue_monthly_recap_emails(worker_state: &WorkerState) -> anyhow::Result<usize> {
    // Find all users who have opted in for monthly recap
    let users_with_prefs = email_preferences::Entity::find()
        .filter(email_preferences::Column::MonthlyRecap.eq(true))
        .find_also_related(users::Entity)
        .all(&worker_state.db)
        .await?;

    let mut count = 0;

    for (_pref, user_opt) in users_with_prefs {
        if let Some(user) = user_opt {
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
