use chrono::Utc;

use crate::refresh_token::Model;

impl Model {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
