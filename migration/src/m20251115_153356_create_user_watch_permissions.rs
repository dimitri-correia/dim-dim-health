use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserWatchPermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserWatchPermissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(UserWatchPermissions::UserWatchedId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserWatchPermissions::UserWatchingId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserWatchPermissions::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_watch_permissions_user_watched_id")
                            .from(
                                UserWatchPermissions::Table,
                                UserWatchPermissions::UserWatchedId,
                            )
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_watch_permissions_user_watching_id")
                            .from(
                                UserWatchPermissions::Table,
                                UserWatchPermissions::UserWatchingId,
                            )
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_watch_permissions_user_watched_id")
                    .table(UserWatchPermissions::Table)
                    .col(UserWatchPermissions::UserWatchedId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_watch_permissions_user_watching_id")
                    .table(UserWatchPermissions::Table)
                    .col(UserWatchPermissions::UserWatchingId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserWatchPermissions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserWatchPermissions {
    Table,
    Id,
    UserWatchedId,
    UserWatchingId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
