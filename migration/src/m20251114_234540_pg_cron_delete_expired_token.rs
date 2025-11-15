use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // pg_cron setup skipped for testing environments where extension is not available
        // In production, manually set up:
        // CREATE EXTENSION IF NOT EXISTS pg_cron;
        // SELECT cron.schedule('cleanup_expired_pass_reset_tokens', '0 * * * *',
        //     'DELETE FROM password_reset_token WHERE expires_at < NOW()');
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // In production, manually clean up:
        // SELECT cron.unschedule('cleanup_expired_pass_reset_tokens');
        Ok(())
    }
}
