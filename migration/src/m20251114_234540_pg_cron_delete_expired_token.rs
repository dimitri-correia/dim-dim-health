use crate::helpers::{schedule_cron_job, unschedule_cron_job};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "cleanup_expired_pass_reset_tokens";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Schedule the cron job to run every day at 01:00 AM
        schedule_cron_job(
            manager,
            CRON_NAME,
            "0 1 * * *",
            "DELETE FROM password_reset_token WHERE expires_at < NOW()",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unschedule_cron_job(manager, CRON_NAME).await
    }
}
