use chrono::NaiveDate;
use entities::gym_session;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct GymSessionRepository {
    db: DatabaseConnection,
}

impl GymSessionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        date: NaiveDate,
    ) -> Result<gym_session::Model, sea_orm::DbErr> {
        let session = gym_session::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            date: Set(date),
            created_at: NotSet,
            updated_at: NotSet,
        };
        let session = session.insert(&self.db).await?;

        Ok(session)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<gym_session::Model>, sea_orm::DbErr> {
        gym_session::Entity::find_by_id(id.to_owned())
            .one(&self.db)
            .await
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<gym_session::Model>, sea_orm::DbErr> {
        gym_session::Entity::find()
            .filter(gym_session::Column::UserId.eq(user_id.to_owned()))
            .order_by_desc(gym_session::Column::Date)
            .all(&self.db)
            .await
    }

    pub async fn find_by_user_and_date(
        &self,
        user_id: &Uuid,
        date: NaiveDate,
    ) -> Result<Vec<gym_session::Model>, sea_orm::DbErr> {
        gym_session::Entity::find()
            .filter(gym_session::Column::UserId.eq(user_id.to_owned()))
            .filter(gym_session::Column::Date.eq(date))
            .all(&self.db)
            .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        date: Option<NaiveDate>,
    ) -> Result<gym_session::Model, sea_orm::DbErr> {
        let mut session: gym_session::ActiveModel = gym_session::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound(
                "Session not found".to_owned(),
            ))?
            .into();

        if let Some(date) = date {
            session.date = Set(date);
        }

        let session = session.update(&self.db).await?;

        Ok(session)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sea_orm::DbErr> {
        gym_session::Entity::delete_by_id(id.to_owned())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
