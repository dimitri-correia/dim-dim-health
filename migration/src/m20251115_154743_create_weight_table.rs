use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserWeight::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserWeight::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(UserWeight::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserWeight::WeightInKg)
                            .decimal_len(5, 2)
                            .not_null()
                            .check(Expr::col(UserWeight::WeightInKg).gte(Expr::value(30.0)))
                            .check(Expr::col(UserWeight::WeightInKg).lte(Expr::value(200.0))),
                    )
                    .col(
                        ColumnDef::new(UserWeight::RecordedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserWeight::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserWeight::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_weight_user_id")
                            .from(UserWeight::Table, UserWeight::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_weight_user_id")
                    .table(UserWeight::Table)
                    .col(UserWeight::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_unique_user_weight_per_day")
                    .table(UserWeight::Table)
                    .col(UserWeight::UserId)
                    .col(UserWeight::RecordedAt)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserWeight::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserWeight {
    Table,
    Id,
    UserId,
    WeightInKg,
    RecordedAt,
    UpdatedAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
