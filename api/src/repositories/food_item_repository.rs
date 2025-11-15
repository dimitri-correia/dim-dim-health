use entities::food_item;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
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
        name: &str,
        description: Option<String>,
        scan_code: Option<String>,
        calories_per100g: i32,
        protein_per100g: i32,
        carbs_per100g: i32,
        fat_per100g: i32,
        added_by: &Uuid,
    ) -> Result<food_item::Model, sea_orm::DbErr> {
        let food_item = food_item::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name.to_string()),
            description: Set(description),
            scan_code: Set(scan_code),
            calories_per100g: Set(calories_per100g),
            protein_per100g: Set(protein_per100g),
            carbs_per100g: Set(carbs_per100g),
            fat_per100g: Set(fat_per100g),
            added_by: Set(added_by.to_owned()),
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

    pub async fn find_by_scan_code(
        &self,
        scan_code: &str,
    ) -> Result<Option<food_item::Model>, sea_orm::DbErr> {
        food_item::Entity::find()
            .filter(food_item::Column::ScanCode.eq(scan_code))
            .one(&self.db)
            .await
    }

    pub async fn search_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<food_item::Model>, sea_orm::DbErr> {
        food_item::Entity::find()
            .filter(food_item::Column::Name.contains(name))
            .order_by_asc(food_item::Column::Name)
            .all(&self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<food_item::Model>, sea_orm::DbErr> {
        food_item::Entity::find()
            .order_by_asc(food_item::Column::Name)
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: &Uuid,
        name: Option<&str>,
        description: Option<Option<String>>,
        scan_code: Option<Option<String>>,
        calories_per100g: Option<i32>,
        protein_per100g: Option<i32>,
        carbs_per100g: Option<i32>,
        fat_per100g: Option<i32>,
    ) -> Result<food_item::Model, sea_orm::DbErr> {
        if name.is_none()
            && description.is_none()
            && scan_code.is_none()
            && calories_per100g.is_none()
            && protein_per100g.is_none()
            && carbs_per100g.is_none()
            && fat_per100g.is_none()
        {
            return Err(sea_orm::DbErr::Custom(
                "At least one field must be provided for update".to_string(),
            ));
        }

        let mut active = food_item::ActiveModel {
            id: Set(id.to_owned()),
            ..Default::default()
        };

        if let Some(n) = name {
            active.name = Set(n.to_string());
        }

        if let Some(desc) = description {
            active.description = Set(desc);
        }

        if let Some(sc) = scan_code {
            active.scan_code = Set(sc);
        }

        if let Some(c) = calories_per100g {
            active.calories_per100g = Set(c);
        }

        if let Some(p) = protein_per100g {
            active.protein_per100g = Set(p);
        }

        if let Some(carb) = carbs_per100g {
            active.carbs_per100g = Set(carb);
        }

        if let Some(f) = fat_per100g {
            active.fat_per100g = Set(f);
        }

        active.update(&self.db).await
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        let food_item = food_item::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await?;

        if let Some(food_item) = food_item {
            let food_item: food_item::ActiveModel = food_item.into();
            food_item.delete(&self.db).await?;
        }

        Ok(())
    }
}
