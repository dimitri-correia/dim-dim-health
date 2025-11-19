use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "cleanup_expired_pass_reset_tokens";
static PASSWORD_RESET_TABLE: &str = "password_reset_token";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Schedule the cron job to run every day at 01:00 AM
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                 SELECT cron.schedule(
                    '{CRON_NAME}',
                    '0 1 * * *',
                    'DELETE FROM {PASSWORD_RESET_TABLE} WHERE expires_at < NOW()'
                );
                "#,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(&format!("SELECT cron.unschedule('{CRON_NAME}');"))
            .await
            .ok(); // ignore errors if not exists

        Ok(())
    }
}
