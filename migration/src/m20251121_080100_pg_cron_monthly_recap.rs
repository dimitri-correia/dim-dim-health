use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "send_monthly_recap_emails";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create a stored procedure to enqueue monthly recap jobs
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION enqueue_monthly_recap_emails()
                RETURNS void AS $$
                BEGIN
                    -- Insert users who have opted in for monthly recap into the queue
                    INSERT INTO monthly_recap_queue (user_id, processed)
                    SELECT ep.user_id, false
                    FROM email_preferences ep
                    WHERE ep.monthly_recap = true
                    ON CONFLICT DO NOTHING;
                    
                    RAISE NOTICE 'Monthly recap email jobs enqueued at %', NOW();
                END;
                $$ LANGUAGE plpgsql;
                "#,
            )
            .await?;

        // Schedule the cron job to run on the 1st of each month at 09:00 AM
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                 SELECT cron.schedule(
                    '{CRON_NAME}',
                    '0 9 1 * *',
                    'SELECT enqueue_monthly_recap_emails()'
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

        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS enqueue_monthly_recap_emails();")
            .await
            .ok();

        Ok(())
    }
}
