use crate::helpers::{schedule_cron_job, unschedule_cron_job};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "cleanup_expired_tokens";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Ensure pg_cron extension is enabled
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_cron;")
            .await?;

        // Schedule the cron job to run every day at 02:00 AM
        schedule_cron_job(
            manager,
            CRON_NAME,
            "0 2 * * *",
            "DELETE FROM email_verification_token WHERE expires_at < NOW()",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unschedule_cron_job(manager, CRON_NAME).await
    }
}
