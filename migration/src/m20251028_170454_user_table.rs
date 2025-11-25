use crate::helpers::{drop_updated_at_trigger, schedule_cron_job, unschedule_cron_job};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

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
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string_len(20)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::PasswordHash)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::EmailVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::ProfileImage)
                            .custom(Alias::new(USER_PROFILE_IMAGE_ENUM))
                            .not_null()
                            .default("avatar1"),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_users_username")
                    .table(Users::Table)
                    .col(Users::Username)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_users_created_at")
                    .table(Users::Table)
                    .col(Users::CreatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_users_email_verified")
                    .table(Users::Table)
                    .col(Users::EmailVerified)
                    .to_owned(),
            )
            .await?;

        // Add trigger for updated_at
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = NOW();
                    RETURN NEW;
                END;
                $$ language 'plpgsql';
                
                CREATE TRIGGER update_users_updated_at
                    BEFORE UPDATE ON users
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        // Schedule the cron job to clean non-verified email users
        // We keep non-verified email users for 5 days before cleaning them up
        // It's a bit more than the time they have to verify their email
        schedule_cron_job(
            manager,
            CRON_NAME,
            "30 0 * * *",
            &format!(
                "DELETE FROM {USER_TABLE} WHERE email_verified = FALSE AND created_at < NOW() - INTERVAL '5 days'"
            ),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Unschedule the cron job
        unschedule_cron_job(manager, CRON_NAME).await?;

        // Drop trigger
        drop_updated_at_trigger(manager, "users").await?;

        // Drop the shared function (since this is the first migration that creates it)
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_column;")
            .await?;

        // Drop enum type
        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", USER_PROFILE_IMAGE_ENUM))
            .await?;

        // Drop table
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

static USER_PROFILE_IMAGE_ENUM: &str = "user_profile_image";
static CRON_NAME: &str = "cleanup_expired_non_verified_emails";
static USER_TABLE: &str = "users";

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    EmailVerified,
    ProfileImage,
    CreatedAt,
    UpdatedAt,
}
