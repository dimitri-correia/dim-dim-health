use entities::JobEmailYearlyRecap;
use tracing::info;

use crate::{mail_jobs::common_mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_yearly_recap_email(
    worker_state: WorkerState,
    data: JobEmailYearlyRecap,
) -> anyhow::Result<bool> {
    info!("Handling yearly recap email for: {}", data.email);
    let subject = format!("DimDim Health - Your Yearly Recap, {}", data.username);
    let preferences_url = format!("{}/settings/email-preferences", worker_state.frontend_url);
    let content = format!(
        "Hey {}.\n\nHappy New Year! 🎉 This is your yearly recap for DimDim Health!\n\n[PLACEHOLDER: Yearly statistics and progress will be displayed here]\n\nYour achievements this year:\n- Total workouts completed: [PLACEHOLDER]\n- Total weight change: [PLACEHOLDER]\n- Total meals logged: [PLACEHOLDER]\n- Most active month: [PLACEHOLDER]\n- Longest streak: [PLACEHOLDER]\n\nCongratulations on another year of health progress!\n\nCheers,\nDimDim Health Team\n\n---\nManage your email preferences: {}",
        data.username,
        preferences_url
    );

    send_email(
        worker_state,
        data.email.to_string(),
        subject.to_string(),
        content.to_string(),
    )
    .await
}
