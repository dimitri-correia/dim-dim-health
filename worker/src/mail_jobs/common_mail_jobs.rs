use crate::{
    mail_jobs::{
        register_mail::handle_registration_email, reset_password_mail::handle_reset_password_email,
        monthly_recap_mail::handle_monthly_recap_email, weekly_recap_mail::handle_weekly_recap_email,
        yearly_recap_mail::handle_yearly_recap_email,
    },
    worker_main::state::WorkerState,
};
use entities::{EmailType, JobEmail, JobEmailRegister, JobEmailResetPassword, JobEmailMonthlyRecap, JobEmailWeeklyRecap, JobEmailYearlyRecap};
use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use tracing::info;

pub async fn handle_mail_job(worker_state: WorkerState, job: JobEmail) -> anyhow::Result<bool> {
    match job.email_type {
        EmailType::Registration => {
            let payload: JobEmailRegister = serde_json::from_value(job.data)?;
            handle_registration_email(worker_state, payload).await
        }
        EmailType::ResetPassword => {
            let payload: JobEmailResetPassword = serde_json::from_value(job.data)?;
            handle_reset_password_email(worker_state, payload).await
        }
        EmailType::MonthlyRecap => {
            let payload: JobEmailMonthlyRecap = serde_json::from_value(job.data)?;
            handle_monthly_recap_email(worker_state, payload).await
        }
        EmailType::WeeklyRecap => {
            let payload: JobEmailWeeklyRecap = serde_json::from_value(job.data)?;
            handle_weekly_recap_email(worker_state, payload).await
        }
        EmailType::YearlyRecap => {
            let payload: JobEmailYearlyRecap = serde_json::from_value(job.data)?;
            handle_yearly_recap_email(worker_state, payload).await
        }
    }
}

pub async fn send_email(
    worker_state: WorkerState,
    to: String,
    subject: String,
    content: String,
) -> anyhow::Result<bool> {
    info!("Sending email [{}] to: {}", subject, to);

    let email = Message::builder()
        .from(worker_state.gmail_from.clone())
        .to(to
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse to address: {}", e))?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(content)
        .map_err(|e| anyhow::anyhow!("Failed to build email: {}", e))?;

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| anyhow::anyhow!("Failed to create SMTP transport: {}", e))?
        .credentials(worker_state.gmail_creds.clone())
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            info!("Email sent successfully!");
            Ok(true)
        }
        Err(e) => {
            info!("Failed to send email: {}", e);
            Ok(false)
        }
    }
}
