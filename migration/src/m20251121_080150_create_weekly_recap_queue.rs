use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WeeklyRecapQueue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WeeklyRecapQueue::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(WeeklyRecapQueue::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WeeklyRecapQueue::Processed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(WeeklyRecapQueue::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(WeeklyRecapQueue::ProcessedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_weekly_recap_queue_user_id")
                            .from(WeeklyRecapQueue::Table, WeeklyRecapQueue::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for efficient querying of unprocessed jobs
        manager
            .create_index(
                Index::create()
                    .name("idx_weekly_recap_queue_processed")
                    .table(WeeklyRecapQueue::Table)
                    .col(WeeklyRecapQueue::Processed)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WeeklyRecapQueue::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum WeeklyRecapQueue {
    Table,
    Id,
    UserId,
    Processed,
    CreatedAt,
    ProcessedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
