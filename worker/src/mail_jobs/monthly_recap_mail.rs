use entities::JobEmailMonthlyRecap;
use tracing::info;

use crate::{mail_jobs::common_mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_monthly_recap_email(
    worker_state: WorkerState,
    data: JobEmailMonthlyRecap,
) -> anyhow::Result<bool> {
    info!("Handling monthly recap email for: {}", data.email);
    let subject = format!("DimDim Health - Your Monthly Recap, {}", data.username);
    let content = format!(
        "Hey {}.\n\nThis is your monthly recap for DimDim Health!\n\n[PLACEHOLDER: Monthly statistics and progress will be displayed here]\n\nKey achievements this month:\n- Total workouts: [PLACEHOLDER]\n- Weight change: [PLACEHOLDER]\n- Meals logged: [PLACEHOLDER]\n\nKeep up the great work!\n\nCheers,\nDimDim Health Team",
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
