use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static MEAL_TYPE_ENUM: &str = "meal_type_enum";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(&format!(
                "CREATE TYPE {} AS ENUM ('breakfast', 'lunch', 'snack', 'dinner');",
                MEAL_TYPE_ENUM
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Meal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Meal::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Meal::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Meal::Kind)
                            .custom(Alias::new(MEAL_TYPE_ENUM))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Meal::Date).date().not_null())
                    .col(ColumnDef::new(Meal::Description).text().null())
                    .col(
                        ColumnDef::new(Meal::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Meal::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_meal_user_id")
                            .from(Meal::Table, Meal::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_meal_user_id")
                    .table(Meal::Table)
                    .col(Meal::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Meal::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", MEAL_TYPE_ENUM))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Meal {
    Table,
    Id,
    UserId,
    Kind,
    Date,
    Description,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
