use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "cleanup_expired_tokens";
static EMAIL_VERIFICATION_TABLE: &str = "email_verification_token";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                CREATE EXTENSION IF NOT EXISTS pg_cron;
                 SELECT cron.schedule(
                    '{CRON_NAME}',
                    '0 * * * *',
                    'DELETE FROM {EMAIL_VERIFICATION_TABLE} WHERE expires_at < NOW()'
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
