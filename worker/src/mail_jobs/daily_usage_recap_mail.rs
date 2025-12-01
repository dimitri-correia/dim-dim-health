use entities::JobEmailDailyUsageRecap;
use tracing::info;

use crate::{mail_jobs::common_mail_jobs::send_email, worker_main::state::WorkerState};

pub async fn handle_daily_usage_recap_email(
    worker_state: WorkerState,
    data: JobEmailDailyUsageRecap,
) -> anyhow::Result<bool> {
    info!("Handling daily usage recap email for: {}", data.email);
    let subject = format!("DimDim Health - Daily Usage Recap for {}", data.date);
    let content = format!(
        "Daily Usage Recap for {}\n\n{}\n\nCheers,\nDimDim Health Team",
        data.date, data.usage_summary
    );

    send_email(
        worker_state,
        data.email.to_string(),
        subject.to_string(),
        content.to_string(),
    )
    .await
}
