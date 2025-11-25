use crate::helpers::{schedule_cron_job, unschedule_cron_job};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "cleanup_expired_non_verified_emails";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // We keep non-verified email users for 5 days before cleaning them up
        // It's a bit more than the time they have to verify their email
        // (see pg cron delete expired token migration)
        let clean_cmd = r#"
            DELETE FROM users
            WHERE email_verified = FALSE
            AND created_at < NOW() - INTERVAL '5 days';
        "#;

        // Schedule the cron job to run every day at 00:30 AM
        schedule_cron_job(manager, CRON_NAME, "30 0 * * *", clean_cmd).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unschedule_cron_job(manager, CRON_NAME).await
    }
}
