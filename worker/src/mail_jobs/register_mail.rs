use entities::JobEmailRegister;

use crate::{mail_jobs::mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_registration_email(
    worker_state: WorkerState,
    data: JobEmailRegister,
) -> anyhow::Result<bool> {
    let subject = "aa";
    let content = "aa";
    send_email(
        worker_state,
        data.email.to_string(),
        subject.to_string(),
        content.to_string(),
    )
    .await
}
