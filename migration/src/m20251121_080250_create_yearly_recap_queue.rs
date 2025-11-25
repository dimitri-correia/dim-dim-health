use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(YearlyRecapQueue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(YearlyRecapQueue::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(YearlyRecapQueue::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(YearlyRecapQueue::Processed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(YearlyRecapQueue::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(YearlyRecapQueue::ProcessedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_yearly_recap_queue_user_id")
                            .from(YearlyRecapQueue::Table, YearlyRecapQueue::UserId)
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
                    .name("idx_yearly_recap_queue_processed")
                    .table(YearlyRecapQueue::Table)
                    .col(YearlyRecapQueue::Processed)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(YearlyRecapQueue::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum YearlyRecapQueue {
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
