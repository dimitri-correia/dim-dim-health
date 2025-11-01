use super::email_verification_token::Model;
use chrono::Utc;

impl Model {
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }
}
