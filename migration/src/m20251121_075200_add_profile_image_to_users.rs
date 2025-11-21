use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the user_profile_image enum type
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TYPE user_profile_image AS ENUM (
                    'avatar1',
                    'avatar2',
                    'avatar3',
                    'avatar4',
                    'avatar5'
                );
                "#,
            )
            .await?;

        // Add the profile_image column to users table with default value
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Users::ProfileImage)
                            .custom(Alias::new("user_profile_image"))
                            .not_null()
                            .default("avatar1"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the column
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::ProfileImage)
                    .to_owned(),
            )
            .await?;

        // Drop the enum type
        manager
            .get_connection()
            .execute_unprepared("DROP TYPE IF EXISTS user_profile_image;")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    ProfileImage,
}
