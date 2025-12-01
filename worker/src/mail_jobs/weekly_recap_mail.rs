use entities::JobEmailWeeklyRecap;
use tracing::info;

use crate::{mail_jobs::common_mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_weekly_recap_email(
    worker_state: WorkerState,
    data: JobEmailWeeklyRecap,
) -> anyhow::Result<bool> {
    info!("Handling weekly recap email for: {}", data.email);
    let subject = format!("DimDim Health - Your Weekly Recap, {}", data.username);
    let preferences_url = format!("{}/settings/email-preferences", worker_state.frontend_url);
    let content = format!(
        "Hey {}.\n\nThis is your weekly recap for DimDim Health!\n\n[PLACEHOLDER: Weekly statistics and progress will be displayed here]\n\nThis week's highlights:\n- Workouts completed: [PLACEHOLDER]\n- Weight change: [PLACEHOLDER]\n- Meals logged: [PLACEHOLDER]\n- Daily average calories: [PLACEHOLDER]\n\nKeep up the great work!\n\nCheers,\nDimDim Health Team\n\n---\nManage your email preferences: {}",
        data.username, preferences_url
    );

    send_email(
        worker_state,
        data.email.to_string(),
        subject.to_string(),
        content.to_string(),
    )
    .await
}
