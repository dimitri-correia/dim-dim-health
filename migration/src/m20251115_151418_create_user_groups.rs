use crate::helpers::{schedule_cron_job, unschedule_cron_job};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static USER_GROUP_ENUM: &str = "user_group";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(&format!(
                "CREATE TYPE {} AS ENUM ('admin_group', 'public_group', 'guest_group');",
                USER_GROUP_ENUM
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserGroups::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserGroups::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserGroups::Group)
                            .custom(Alias::new(USER_GROUP_ENUM))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserGroups::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(UserGroups::UserId)
                            .col(UserGroups::Group),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_groups_user_id")
                            .from(UserGroups::Table, UserGroups::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_groups_user_id")
                    .table(UserGroups::Table)
                    .col(UserGroups::UserId)
                    .to_owned(),
            )
            .await?;

        // Schedule the cron job to clean expired guest users
        let clean_cmd = format!(
            r#"
            DELETE FROM {USER_TABLE}
            WHERE id IN (
                SELECT ug.user_id
                FROM {USER_GROUPS_TABLE} ug
                WHERE ug.group = 'guest_group'
                AND ug.expires_at < NOW() - INTERVAL '24 hours'
            );
            "#
        );
        schedule_cron_job(manager, CRON_NAME, "0 0 * * *", &clean_cmd).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unschedule_cron_job(manager, CRON_NAME).await?;

        manager
            .drop_table(Table::drop().table(UserGroups::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", USER_GROUP_ENUM))
            .await?;

        Ok(())
    }
}

static CRON_NAME: &str = "cleanup_expired_guests_users";
static USER_GROUPS_TABLE: &str = "user_groups";
static USER_TABLE: &str = "users";

#[derive(DeriveIden)]
enum UserGroups {
    Table,
    UserId,
    Group,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
