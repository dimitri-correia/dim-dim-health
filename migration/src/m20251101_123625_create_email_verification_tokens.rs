use crate::helpers::{schedule_cron_job, unschedule_cron_job};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailVerificationToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailVerificationToken::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(EmailVerificationToken::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerificationToken::Token)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(EmailVerificationToken::PendingEmail)
                            .string_len(255)
                            .null(),
                    )
                    // Add pending_email column to email_verification_token table
                    // This will store the new email when a user changes their email
                    // NULL means it's a regular email verification (for new users)
                    // Non-NULL means it's an email change verification
                    .col(
                        ColumnDef::new(EmailVerificationToken::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerificationToken::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_email_verification_tokens_user_id")
                            .from(
                                EmailVerificationToken::Table,
                                EmailVerificationToken::UserId,
                            )
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_email_verification_tokens_token")
                    .table(EmailVerificationToken::Table)
                    .col(EmailVerificationToken::Token)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_email_verification_tokens_user_id")
                    .table(EmailVerificationToken::Table)
                    .col(EmailVerificationToken::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_email_verification_tokens_expires_at")
                    .table(EmailVerificationToken::Table)
                    .col(EmailVerificationToken::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        // Schedule the cron job to run every day at 02:00 AM
        // Ensure pg_cron extension is enabled
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_cron;")
            .await?;

        schedule_cron_job(
            manager,
            CRON_NAME,
            "0 2 * * *",
            &format!("DELETE FROM {EMAIL_VERIFICATION_TABLE} WHERE expires_at < NOW()"),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unschedule_cron_job(manager, CRON_NAME).await?;

        manager
            .drop_table(
                Table::drop()
                    .table(EmailVerificationToken::Table)
                    .to_owned(),
            )
            .await
    }
}

static CRON_NAME: &str = "cleanup_expired_tokens";
static EMAIL_VERIFICATION_TABLE: &str = "email_verification_token";

#[derive(DeriveIden)]
enum EmailVerificationToken {
    Table,
    Id,
    UserId,
    Token,
    PendingEmail,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
