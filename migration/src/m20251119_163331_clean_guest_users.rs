use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static CRON_NAME: &str = "cleanup_expired_guests_users";
static USER_GROUPS_TABLE: &str = "user_groups";
static USER_TABLE: &str = "users";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let clean_cmd = format!(
            r#"
            DELETE FROM {USER_TABLE}
            WHERE id IN (
                SELECT ug.user_id
                FROM {USER_GROUPS_TABLE} ug
                WHERE ug.group = 'guest_group'
                AND ug.expires_at < NOW() - INTERVAL '24 hours'
            );
        "#
        );
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                CREATE EXTENSION IF NOT EXISTS pg_cron;
                 SELECT cron.schedule(
                    '{CRON_NAME}',
                    '0 * * * *',
                    '{clean_cmd}'
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
