use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add pending_email column to email_verification_token table
        // This will store the new email when a user changes their email
        // NULL means it's a regular email verification (for new users)
        // Non-NULL means it's an email change verification
        manager
            .alter_table(
                Table::alter()
                    .table(EmailVerificationToken::Table)
                    .add_column(
                        ColumnDef::new(EmailVerificationToken::PendingEmail)
                            .string_len(255)
                            .null(),
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
                    .table(EmailVerificationToken::Table)
                    .drop_column(EmailVerificationToken::PendingEmail)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EmailVerificationToken {
    Table,
    PendingEmail,
}
