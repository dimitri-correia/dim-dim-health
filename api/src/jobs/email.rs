use entities::{EmailType, Job, JobEmail, JobEmailRegister, TaskType};
use redis::{AsyncCommands, aio::ConnectionManager};

pub async fn send_register_email(
    email: &str,
    username: &str,
    token: &str,
    con: ConnectionManager,
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

    let mut con = con.clone();
    con.rpush::<_, _, ()>("jobs", serde_json::to_string(&job).unwrap())
        .await
}
