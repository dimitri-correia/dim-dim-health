use entities::{meal, sea_orm_active_enums::MealTypeEnum};
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
        user_id: &Uuid,
        kind: MealTypeEnum,
        date: chrono::NaiveDate,
        description: Option<String>,
    ) -> Result<meal::Model, sea_orm::DbErr> {
        let meal = meal::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id.to_owned()),
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
        meal::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<meal::Model>, sea_orm::DbErr> {
        meal::Entity::find()
            .filter(meal::Column::UserId.eq(user_id.to_owned()))
            .order_by_desc(meal::Column::Date)
            .all(&self.db)
            .await
    }

    pub async fn find_by_user_id_and_date(
        &self,
        user_id: &Uuid,
        date: chrono::NaiveDate,
    ) -> Result<Vec<meal::Model>, sea_orm::DbErr> {
        meal::Entity::find()
            .filter(meal::Column::UserId.eq(user_id.to_owned()))
            .filter(meal::Column::Date.eq(date))
            .order_by_asc(meal::Column::Kind)
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: &Uuid,
        kind: Option<MealTypeEnum>,
        date: Option<chrono::NaiveDate>,
        description: Option<Option<String>>,
    ) -> Result<meal::Model, sea_orm::DbErr> {
        if kind.is_none() && date.is_none() && description.is_none() {
            return Err(sea_orm::DbErr::Custom(
                "At least one field must be provided for update".to_string(),
            ));
        }

        let mut active = meal::ActiveModel {
            id: Set(id.to_owned()),
            ..Default::default()
        };

        if let Some(k) = kind {
            active.kind = Set(k);
        }

        if let Some(d) = date {
            active.date = Set(d);
        }

        if let Some(desc) = description {
            active.description = Set(desc);
        }

        active.update(&self.db).await
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        let meal = meal::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await?;

        if let Some(meal) = meal {
            let meal: meal::ActiveModel = meal.into();
            meal.delete(&self.db).await?;
        }

        Ok(())
    }
}
