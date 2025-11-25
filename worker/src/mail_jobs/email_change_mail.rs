use crate::{mail_jobs::common_mail_jobs::send_email, worker_main::state::WorkerState};
use entities::JobEmailRegister;

pub async fn handle_email_change_email(
    worker_state: WorkerState,
    payload: JobEmailRegister,
) -> anyhow::Result<bool> {
    let subject = "Verify your new email address".to_string();
    let content = format!(
        "Hi {},\n\n\
        You have requested to change your email address.\n\n\
        Please click on the following link to verify your new email address:\n\
        {}/#/verify-email?token={}\n\n\
        This link will expire in 2 hours.\n\n\
        Note: Your login email will remain the same until you verify the new email.\n\n\
        If you didn't request this change, please ignore this email.\n\n\
        Best regards,\n\
        DimDim Health Team",
        payload.username, worker_state.frontend_url, payload.token
    );

    send_email(worker_state, payload.email, subject, content).await
}
