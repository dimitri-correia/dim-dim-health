use entities::food_item;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct FoodItemRepository {
    db: DatabaseConnection,
}

impl FoodItemRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        scan_code: Option<String>,
        calories_per100g: i32,
        protein_per100g: i32,
        carbs_per100g: i32,
        fat_per100g: i32,
        added_by: Uuid,
    ) -> Result<food_item::Model, sea_orm::DbErr> {
        let food_item = food_item::ActiveModel {
            id: NotSet,
            name: Set(name),
            description: Set(description),
            scan_code: Set(scan_code),
            calories_per100g: Set(calories_per100g),
            protein_per100g: Set(protein_per100g),
            carbs_per100g: Set(carbs_per100g),
            fat_per100g: Set(fat_per100g),
            added_by: Set(added_by),
            added_at: NotSet,
        };
        let food_item = food_item.insert(&self.db).await?;

        Ok(food_item)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<food_item::Model>, sea_orm::DbErr> {
        food_item::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<food_item::Model>, sea_orm::DbErr> {
        food_item::Entity::find().all(&self.db).await
    }

    pub async fn find_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<food_item::Model>, sea_orm::DbErr> {
        food_item::Entity::find()
            .filter(food_item::Column::Name.contains(name))
            .all(&self.db)
            .await
    }

    pub async fn find_by_scan_code(
        &self,
        scan_code: &str,
    ) -> Result<Option<food_item::Model>, sea_orm::DbErr> {
        food_item::Entity::find()
            .filter(food_item::Column::ScanCode.eq(scan_code.to_owned()))
            .one(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<Option<String>>,
        scan_code: Option<Option<String>>,
        calories_per100g: Option<i32>,
        protein_per100g: Option<i32>,
        carbs_per100g: Option<i32>,
        fat_per100g: Option<i32>,
    ) -> Result<food_item::Model, sea_orm::DbErr> {
        let mut food_item: food_item::ActiveModel = food_item::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Food item not found".to_owned(),
            ))?
            .into();

        if let Some(name) = name {
            food_item.name = Set(name);
        }
        if let Some(description) = description {
            food_item.description = Set(description);
        }
        if let Some(scan_code) = scan_code {
            food_item.scan_code = Set(scan_code);
        }
        if let Some(calories) = calories_per100g {
            food_item.calories_per100g = Set(calories);
        }
        if let Some(protein) = protein_per100g {
            food_item.protein_per100g = Set(protein);
        }
        if let Some(carbs) = carbs_per100g {
            food_item.carbs_per100g = Set(carbs);
        }
        if let Some(fat) = fat_per100g {
            food_item.fat_per100g = Set(fat);
        }

        let food_item = food_item.update(&self.db).await?;

        Ok(food_item)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        food_item::Entity::delete_by_id(id.to_owned())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
