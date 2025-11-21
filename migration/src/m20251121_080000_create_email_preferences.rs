use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailPreferences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailPreferences::UserId)
                            .primary_key()
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailPreferences::MonthlyRecap)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(EmailPreferences::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(EmailPreferences::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_email_preferences_user_id")
                            .from(EmailPreferences::Table, EmailPreferences::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add trigger for updated_at
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_email_preferences_updated_at
                    BEFORE UPDATE ON email_preferences
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop trigger
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DROP TRIGGER IF EXISTS update_email_preferences_updated_at ON email_preferences;
                "#,
            )
            .await?;

        // Drop table
        manager
            .drop_table(Table::drop().table(EmailPreferences::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EmailPreferences {
    Table,
    UserId,
    MonthlyRecap,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
