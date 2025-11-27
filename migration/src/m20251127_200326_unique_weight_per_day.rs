use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create a unique index on user_id and the date portion of recorded_at
        // This ensures that a user can only have one weight record per day
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE UNIQUE INDEX idx_unique_user_weight_per_day
                ON user_weight (user_id, CAST(recorded_at AS DATE));
                "#,
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DROP INDEX IF EXISTS idx_unique_user_weight_per_day;
                "#,
            )
            .await?;
        Ok(())
    }
}
