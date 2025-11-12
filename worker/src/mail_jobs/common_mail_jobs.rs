use crate::{mail_jobs::register_mail::handle_registration_email, worker_main::state::WorkerState};
use entities::{EmailType, JobEmail, JobEmailRegister};
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use tracing::info;

pub async fn handle_mail_job(worker_state: WorkerState, job: JobEmail) -> anyhow::Result<bool> {
    match job.email_type {
        EmailType::Registration => {
            let payload: JobEmailRegister = serde_json::from_value(job.data)?;
            handle_registration_email(worker_state, payload).await
        }
    }
}

pub async fn send_email(
    worker_state: WorkerState,
    to: String,
    subject: String,
    content: String,
) -> anyhow::Result<bool> {
    info!("Sending email to: {}", to);
    info!("Subject: {}", subject);

    // Build the email message
    let email = Message::builder()
        .from(
            format!("DimDim Health <{}>", worker_state.gmail_email)
                .parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse from address: {}", e))?,
        )
        .to(to
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse to address: {}", e))?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(content)
        .map_err(|e| anyhow::anyhow!("Failed to build email: {}", e))?;

    // Create SMTP credentials
    let creds = Credentials::new(
        worker_state.gmail_email.clone(),
        worker_state.gmail_password.clone(),
    );

    // Build the SMTP transport for Gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| anyhow::anyhow!("Failed to create SMTP transport: {}", e))?
        .credentials(creds)
        .build();

    // Send the email
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
