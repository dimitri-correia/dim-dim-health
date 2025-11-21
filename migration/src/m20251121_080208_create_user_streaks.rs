use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static SUB_APP_TYPE_ENUM: &str = "sub_app_type_enum";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(&format!(
                "CREATE TYPE {} AS ENUM ('weight', 'diet', 'workout');",
                SUB_APP_TYPE_ENUM
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserStreaks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserStreaks::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(UserStreaks::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserStreaks::SubApp)
                            .custom(Alias::new(SUB_APP_TYPE_ENUM))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserStreaks::CurrentStreak)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStreaks::LongestStreak)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStreaks::LastActivityWeek)
                            .date()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserStreaks::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_streaks_user_id")
                            .from(UserStreaks::Table, UserStreaks::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_streaks_user_id")
                    .table(UserStreaks::Table)
                    .col(UserStreaks::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_streaks_user_sub_app")
                    .table(UserStreaks::Table)
                    .col(UserStreaks::UserId)
                    .col(UserStreaks::SubApp)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserStreaks::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", SUB_APP_TYPE_ENUM))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserStreaks {
    Table,
    Id,
    UserId,
    SubApp,
    CurrentStreak,
    LongestStreak,
    LastActivityWeek,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
