use crate::helpers::{schedule_cron_job, unschedule_cron_job};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "cleanup_expired_guests_users";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let clean_cmd = r#"
            DELETE FROM users
            WHERE id IN (
                SELECT ug.user_id
                FROM user_groups ug
                WHERE ug.group = 'guest_group'
                AND ug.expires_at < NOW() - INTERVAL '24 hours'
            );
        "#;

        // Schedule the cron job to run every day at midnight
        schedule_cron_job(manager, CRON_NAME, "0 0 * * *", clean_cmd).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unschedule_cron_job(manager, CRON_NAME).await
    }
}
