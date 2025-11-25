use entities::JobEmailRegister;
use tracing::info;

use crate::{mail_jobs::common_mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_registration_email(
    worker_state: WorkerState,
    data: JobEmailRegister,
) -> anyhow::Result<bool> {
    info!("Handling registration email for: {}", data.email);
    let subject = format!("DimDim Health - Verify your email {}", data.username);
    let verification_link = format!(
        "{}/#/verify-email?token={}",
        worker_state.frontend_url, data.token
    );
    let content = format!(
        "Hey {}.\nThanks for registering!\nPlease verify your email by clicking the following link: {verification_link} (this link will expire in 2 hours)\n\n Cheers,\n DimDim Health",
        data.username
    );

    send_email(worker_state, data.email, subject, content).await
}
