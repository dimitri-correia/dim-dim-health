use chrono::Utc;

use crate::email_verification_token::Model;

impl Model {
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }
}
