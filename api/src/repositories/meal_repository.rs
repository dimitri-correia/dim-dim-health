use entities::meal;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct MealRepository {
    db: DatabaseConnection,
}

impl MealRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        kind: entities::sea_orm_active_enums::MealTypeEnum,
        date: chrono::NaiveDate,
        description: Option<String>,
    ) -> Result<meal::Model, sea_orm::DbErr> {
        let meal = meal::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            kind: Set(kind),
            date: Set(date),
            description: Set(description),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let meal = meal.insert(&self.db).await?;

        Ok(meal)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<meal::Model>, sea_orm::DbErr> {
        meal::Entity::find_by_id(*id).one(&self.db).await
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<meal::Model>, sea_orm::DbErr> {
        meal::Entity::find()
            .filter(meal::Column::UserId.eq(*user_id))
            .order_by_desc(meal::Column::Date)
            .all(&self.db)
            .await
    }

    pub async fn find_by_user_and_date(
        &self,
        user_id: &Uuid,
        date: chrono::NaiveDate,
    ) -> Result<Vec<meal::Model>, sea_orm::DbErr> {
        meal::Entity::find()
            .filter(meal::Column::UserId.eq(*user_id))
            .filter(meal::Column::Date.eq(date))
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        kind: Option<entities::sea_orm_active_enums::MealTypeEnum>,
        date: Option<chrono::NaiveDate>,
        description: Option<Option<String>>,
    ) -> Result<meal::Model, sea_orm::DbErr> {
        let mut meal: meal::ActiveModel = meal::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound("Meal not found".to_owned()))?
            .into();

        if let Some(kind) = kind {
            meal.kind = Set(kind);
        }
        if let Some(date) = date {
            meal.date = Set(date);
        }
        if let Some(description) = description {
            meal.description = Set(description);
        }

        let meal = meal.update(&self.db).await?;

        Ok(meal)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        meal::Entity::delete_by_id(*id).exec(&self.db).await?;
        Ok(())
    }
}
