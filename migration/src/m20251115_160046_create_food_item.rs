use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FoodItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FoodItem::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(FoodItem::Name).string_len(255).not_null())
                    .col(ColumnDef::new(FoodItem::Description).text().null())
                    .col(ColumnDef::new(FoodItem::ScanCode).string_len(255).null())
                    .col(
                        ColumnDef::new(FoodItem::CaloriesPer100g)
                            .integer()
                            .not_null()
                            .check(Expr::col(FoodItem::CaloriesPer100g).gte(Expr::value(0.0))),
                    )
                    .col(
                        ColumnDef::new(FoodItem::ProteinPer100g)
                            .integer()
                            .not_null()
                            .check(Expr::col(FoodItem::ProteinPer100g).gte(Expr::value(0.0))),
                    )
                    .col(
                        ColumnDef::new(FoodItem::CarbsPer100g)
                            .integer()
                            .not_null()
                            .check(Expr::col(FoodItem::CarbsPer100g).gte(Expr::value(0.0))),
                    )
                    .col(
                        ColumnDef::new(FoodItem::FatPer100g)
                            .integer()
                            .not_null()
                            .check(Expr::col(FoodItem::FatPer100g).gte(Expr::value(0.0))),
                    )
                    .col(ColumnDef::new(FoodItem::AddedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(FoodItem::AddedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_food_item_added_by")
                            .from(FoodItem::Table, FoodItem::AddedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_food_item_added_by")
                    .table(FoodItem::Table)
                    .col(FoodItem::AddedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_food_item_scan_code")
                    .table(FoodItem::Table)
                    .col(FoodItem::ScanCode)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_food_item_name")
                    .table(FoodItem::Table)
                    .col(FoodItem::Name)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FoodItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FoodItem {
    Table,
    Id,
    Name,
    Description,
    ScanCode,
    CaloriesPer100g,
    ProteinPer100g,
    CarbsPer100g,
    FatPer100g,
    AddedBy,
    AddedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
