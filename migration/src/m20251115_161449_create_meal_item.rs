use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MealItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MealItem::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(MealItem::MealId).uuid().not_null())
                    .col(ColumnDef::new(MealItem::FoodItemId).uuid().not_null())
                    .col(
                        ColumnDef::new(MealItem::QuantityInGrams)
                            .integer()
                            .not_null()
                            .check(Expr::col(MealItem::QuantityInGrams).gt(Expr::value(0))),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_meal_item_meal_id")
                            .from(MealItem::Table, MealItem::MealId)
                            .to(Meal::Table, Meal::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_meal_item_food_item_id")
                            .from(MealItem::Table, MealItem::FoodItemId)
                            .to(FoodItem::Table, FoodItem::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MealItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MealItem {
    Table,
    Id,
    MealId,
    FoodItemId,
    QuantityInGrams,
}

#[derive(DeriveIden)]
enum Meal {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum FoodItem {
    Table,
    Id,
}
