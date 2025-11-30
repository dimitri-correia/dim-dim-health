use crate::helpers::{create_updated_at_trigger, drop_updated_at_trigger};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static TABLE_NAME: &str = "gym_session";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GymSession::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GymSession::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(GymSession::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(GymSession::Name)
                            .string_len(255)
                            .null(),
                    )
                    .col(ColumnDef::new(GymSession::Notes).text().null())
                    .col(ColumnDef::new(GymSession::Date).date().not_null())
                    .col(
                        ColumnDef::new(GymSession::DurationMinutes)
                            .integer()
                            .null()
                            .check(Expr::col(GymSession::DurationMinutes).gte(Expr::value(0))),
                    )
                    .col(
                        ColumnDef::new(GymSession::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(GymSession::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_session_user_id")
                            .from(GymSession::Table, GymSession::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_gym_session_user_id")
                    .table(GymSession::Table)
                    .col(GymSession::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_gym_session_date")
                    .table(GymSession::Table)
                    .col(GymSession::Date)
                    .to_owned(),
            )
            .await?;

        // Add trigger for updated_at
        create_updated_at_trigger(manager, TABLE_NAME).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop trigger
        drop_updated_at_trigger(manager, TABLE_NAME).await?;

        manager
            .drop_table(Table::drop().table(GymSession::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GymSession {
    Table,
    Id,
    UserId,
    Name,
    Notes,
    Date,
    DurationMinutes,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
