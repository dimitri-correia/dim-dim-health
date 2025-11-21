use redis::aio::ConnectionManager;

use crate::jobs::email::EmailJob;

pub mod email;

#[derive(Clone)]
pub struct Jobs {
    pub email_job: EmailJob,
}

impl Jobs {
    pub fn new(redis: ConnectionManager) -> Self {
        let email_job = EmailJob::new(redis);
        Jobs { email_job }
    }
}
