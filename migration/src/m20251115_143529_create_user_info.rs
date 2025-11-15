use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static GENDER_ENUM: &str = "gender_enum";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(&format!(
                "CREATE TYPE {} AS ENUM ('male', 'female', 'other');",
                GENDER_ENUM
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserAdditionalInfos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAdditionalInfos::UserId)
                            .primary_key()
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAdditionalInfos::BirthDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAdditionalInfos::HeightInCm)
                            .integer()
                            .not_null()
                            .check(Expr::col(UserAdditionalInfos::HeightInCm).gte(Expr::value(100)))
                            .check(
                                Expr::col(UserAdditionalInfos::HeightInCm).lte(Expr::value(300)),
                            ),
                    )
                    .col(
                        ColumnDef::new(UserAdditionalInfos::Gender)
                            .custom(Alias::new(GENDER_ENUM))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAdditionalInfos::ActivityLevel)
                            .decimal_len(4, 3)
                            .not_null()
                            .check(
                                Expr::col(UserAdditionalInfos::ActivityLevel).gte(Expr::value(1.0)),
                            )
                            .check(
                                Expr::col(UserAdditionalInfos::ActivityLevel).lte(Expr::value(2.0)),
                            ),
                    )
                    .col(
                        ColumnDef::new(UserAdditionalInfos::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_additional_infos_user_id")
                            .from(UserAdditionalInfos::Table, UserAdditionalInfos::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAdditionalInfos::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", GENDER_ENUM))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserAdditionalInfos {
    Table,
    BirthDate,
    HeightInCm,
    Gender,
    ActivityLevel,
    UpdatedAt,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
