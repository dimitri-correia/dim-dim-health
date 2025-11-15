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
                "CREATE TYPE {} AS ENUM ('admin_group', 'public_group');",
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_groups_user_id")
                            .from(UserGroups::Table, UserGroups::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

#[derive(DeriveIden)]
enum UserGroups {
    Table,
    UserId,
    Group,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
