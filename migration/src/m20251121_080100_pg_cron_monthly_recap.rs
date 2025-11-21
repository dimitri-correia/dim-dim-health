use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "send_monthly_recap_emails";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create a SQL command that will enqueue monthly recap email jobs
        // This inserts jobs into Redis via a PostgreSQL function that we'll create
        let cron_cmd = format!(
            r#"
            -- This will be handled by triggering a stored procedure
            -- For now, we'll create a placeholder that can be expanded later
            SELECT 1
            "#
        );

        // Schedule the cron job to run on the 1st of each month at 09:00 AM
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                 SELECT cron.schedule(
                    '{CRON_NAME}',
                    '0 9 1 * *',
                    '{cron_cmd}'
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
