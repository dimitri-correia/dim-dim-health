use chrono::Utc;

use crate::password_reset_token::Model;

impl Model {
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }
}
