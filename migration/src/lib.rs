pub use sea_orm_migration::prelude::*;

mod m20251028_170454_user_table;
mod m20251101_123625_create_email_verification_tokens;
mod m20251101_124651_add_email_verified_to_users;
mod m20251105_173226_pg_cron_delete_expired_token;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251028_170454_user_table::Migration),
            Box::new(m20251101_123625_create_email_verification_tokens::Migration),
            Box::new(m20251101_124651_add_email_verified_to_users::Migration),
            Box::new(m20251105_173226_pg_cron_delete_expired_token::Migration),
        ]
    }
}
