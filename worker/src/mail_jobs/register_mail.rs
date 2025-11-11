use entities::JobEmailRegister;
use tracing::info;

use crate::{mail_jobs::mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_registration_email(
    worker_state: WorkerState,
    data: JobEmailRegister,
) -> anyhow::Result<bool> {
    info!("Handling registration email for: {}", data.email);
    let subject = "DimDim Health - Verify your email";
    let base_url = "http://localhost:3000".to_string();
    let verification_link = format!("{}/api/auth/verify-email?token={}", base_url, data.token);
    let content =
        format!("Please verify your email by clicking the following link: {verification_link}");
    send_email(
        worker_state,
        data.email.to_string(),
        subject.to_string(),
        content.to_string(),
    )
    .await
}
