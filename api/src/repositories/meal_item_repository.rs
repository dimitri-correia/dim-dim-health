use entities::meal_item;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
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
        meal_id: &Uuid,
        food_item_id: &Uuid,
        quantity_in_grams: i32,
    ) -> Result<meal_item::Model, sea_orm::DbErr> {
        let meal_item = meal_item::ActiveModel {
            id: Set(Uuid::new_v4()),
            meal_id: Set(meal_id.to_owned()),
            food_item_id: Set(food_item_id.to_owned()),
            quantity_in_grams: Set(quantity_in_grams),
        };
        let meal_item = meal_item.insert(&self.db).await?;

        Ok(meal_item)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<meal_item::Model>, sea_orm::DbErr> {
        meal_item::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
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
        id: &Uuid,
        food_item_id: Option<&Uuid>,
        quantity_in_grams: Option<i32>,
    ) -> Result<meal_item::Model, sea_orm::DbErr> {
        if food_item_id.is_none() && quantity_in_grams.is_none() {
            return Err(sea_orm::DbErr::Custom(
                "At least one field must be provided for update".to_string(),
            ));
        }

        let mut active = meal_item::ActiveModel {
            id: Set(id.to_owned()),
            ..Default::default()
        };

        if let Some(f) = food_item_id {
            active.food_item_id = Set(f.to_owned());
        }

        if let Some(q) = quantity_in_grams {
            active.quantity_in_grams = Set(q);
        }

        active.update(&self.db).await
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        let meal_item = meal_item::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await?;

        if let Some(meal_item) = meal_item {
            let meal_item: meal_item::ActiveModel = meal_item.into();
            meal_item.delete(&self.db).await?;
        }

        Ok(())
    }
}
