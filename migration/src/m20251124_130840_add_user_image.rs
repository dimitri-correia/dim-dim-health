use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static USER_PROFILE_IMAGE_ENUM: &str = "user_profile_image";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                CREATE TYPE {} AS ENUM (
                    'avatar1',
                    'avatar2',
                    'avatar3',
                    'avatar4',
                    'avatar5'
                );
                "#,
                USER_PROFILE_IMAGE_ENUM
            ))
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Users::ProfileImage)
                            .custom(Alias::new(USER_PROFILE_IMAGE_ENUM))
                            .not_null()
                            .default("avatar1"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::ProfileImage)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", USER_PROFILE_IMAGE_ENUM))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    ProfileImage,
}
