use entities::{EmailType, Job, JobEmail, JobEmailRegister, TaskType};
use redis::{AsyncCommands, aio::ConnectionManager};

#[derive(Clone)]
pub struct EmailJob {
    pub redis: ConnectionManager,
}

impl EmailJob {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn send_register_email(
        &self,
        email: &str,
        username: &str,
        token: &str,
    ) -> Result<(), redis::RedisError> {
        let job_email_register = JobEmailRegister {
            email: email.to_string(),
            username: username.to_string(),
            token: token.to_string(),
        };

        let job_email = JobEmail {
            email_type: EmailType::Registration,
            data: serde_json::to_value(job_email_register).unwrap(),
        };

        let job = Job {
            task_type: TaskType::Email,
            data: serde_json::to_value(job_email).unwrap(),
        };

        let mut con = self.redis.clone();
        con.rpush::<_, _, ()>("jobs", serde_json::to_string(&job).unwrap())
            .await
    }

    pub async fn send_password_reset_email(
        &self,
        email: &str,
        username: &str,
        token: &str,
    ) -> Result<(), redis::RedisError> {
        let job_email_reset_password = JobEmailRegister {
            email: email.to_string(),
            username: username.to_string(),
            token: token.to_string(),
        };

        let job_email = JobEmail {
            email_type: EmailType::ResetPassword,
            data: serde_json::to_value(job_email_reset_password).unwrap(),
        };

        let job = Job {
            task_type: TaskType::Email,
            data: serde_json::to_value(job_email).unwrap(),
        };

        let mut con = self.redis.clone();
        con.rpush::<_, _, ()>("jobs", serde_json::to_string(&job).unwrap())
            .await
    }

    pub async fn send_email_change_email(
        &self,
        email: &str,
        username: &str,
        token: &str,
    ) -> Result<(), redis::RedisError> {
        let job_email_change = JobEmailRegister {
            email: email.to_string(),
            username: username.to_string(),
            token: token.to_string(),
        };

        let job_email = JobEmail {
            email_type: EmailType::EmailChange,
            data: serde_json::to_value(job_email_change).unwrap(),
        };

        let job = Job {
            task_type: TaskType::Email,
            data: serde_json::to_value(job_email).unwrap(),
        };

        let mut con = self.redis.clone();
        con.rpush::<_, _, ()>("jobs", serde_json::to_string(&job).unwrap())
            .await
    }
}
