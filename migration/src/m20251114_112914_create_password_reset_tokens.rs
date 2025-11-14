use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PasswordResetToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordResetToken::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(PasswordResetToken::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(PasswordResetToken::Token)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(PasswordResetToken::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResetToken::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_password_reset_tokens_user_id")
                            .from(PasswordResetToken::Table, PasswordResetToken::UserId)
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
                    .name("idx_password_reset_tokens_token")
                    .table(PasswordResetToken::Table)
                    .col(PasswordResetToken::Token)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_password_reset_tokens_user_id")
                    .table(PasswordResetToken::Table)
                    .col(PasswordResetToken::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_password_reset_tokens_expires_at")
                    .table(PasswordResetToken::Table)
                    .col(PasswordResetToken::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PasswordResetToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PasswordResetToken {
    Table,
    Id,
    UserId,
    Token,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
