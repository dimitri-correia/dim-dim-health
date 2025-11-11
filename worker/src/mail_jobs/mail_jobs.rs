use crate::{mail_jobs::register_mail::handle_registration_email, worker_main::state::WorkerState};
use entities::{EmailType, JobEmail, JobEmailRegister};
use reqwest::blocking::Client;

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
    let client = Client::new();

    let res = client
        .post(format!(
            "https://api.mailgun.net/v3/{}/messages",
            worker_state.mailgun_domain
        ))
        .basic_auth("api", Some(worker_state.mailgun_key))
        .form(&[
            (
                "from",
                format!(
                    "Mailgun Sandbox <postmaster@{}>",
                    worker_state.mailgun_domain
                ),
            ),
            ("to", to),
            ("subject", subject),
            ("text", content),
        ])
        .send()?;

    Ok(res.status().is_success())
}
