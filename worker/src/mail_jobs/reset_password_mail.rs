use entities::JobEmailResetPassword;
use tracing::info;

use crate::{mail_jobs::common_mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_reset_password_email(
    worker_state: WorkerState,
    data: JobEmailResetPassword,
) -> anyhow::Result<bool> {
    info!("Handling reset password email for: {}", data.email);
    let subject = format!("DimDim Health - Reset your password {}", data.username);
    let reset_link = format!(
        "{}/api/auth/reset-password?token={}",
        worker_state.base_url, data.token
    );
    let content = format!(
        "Hey {}.\nWe received a request to reset your password. If you didn't make this request, you can safely ignore this email.\nPlease reset your password by clicking the following link: {reset_link} (this link will expire in 1 hour)\n\n Cheers,\n DimDim Health",
        data.username
    );

    send_email(
        worker_state,
        data.email.to_string(),
        subject.to_string(),
        content.to_string(),
    )
    .await
}
