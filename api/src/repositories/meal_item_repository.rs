use entities::meal_item;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct MealItemRepository {
    db: DatabaseConnection,
}

impl MealItemRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        meal_id: Uuid,
        food_item_id: Uuid,
        quantity_in_grams: i32,
    ) -> Result<meal_item::Model, sea_orm::DbErr> {
        let meal_item = meal_item::ActiveModel {
            id: NotSet,
            meal_id: Set(meal_id),
            food_item_id: Set(food_item_id),
            quantity_in_grams: Set(quantity_in_grams),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let meal_item = meal_item.insert(&self.db).await?;

        Ok(meal_item)
    }

    pub async fn find_by_meal_id(
        &self,
        meal_id: &Uuid,
    ) -> Result<Vec<meal_item::Model>, sea_orm::DbErr> {
        meal_item::Entity::find()
            .filter(meal_item::Column::MealId.eq(meal_id.to_owned()))
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        quantity_in_grams: i32,
    ) -> Result<meal_item::Model, sea_orm::DbErr> {
        let meal_item = meal_item::ActiveModel {
            id: Set(id),
            quantity_in_grams: Set(quantity_in_grams),
            ..Default::default()
        };
        let meal_item = meal_item.update(&self.db).await?;

        Ok(meal_item)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        meal_item::Entity::delete_by_id(id.to_owned())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
