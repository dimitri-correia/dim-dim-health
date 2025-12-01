use entities::{sea_orm_active_enums::GenderEnum, user_additional_infos};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait,
    prelude::Decimal,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct UserInfoRepository {
    db: DatabaseConnection,
}

impl UserInfoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        user_id: &Uuid,
        birth_date: &chrono::NaiveDate,
        height_in_cm: i32,
        gender: GenderEnum,
        activity_level: Decimal,
    ) -> Result<user_additional_infos::Model, sea_orm::DbErr> {
        let user_infos = user_additional_infos::ActiveModel {
            user_id: Set(*user_id),
            birth_date: Set(*birth_date),
            height_in_cm: Set(height_in_cm),
            gender: Set(gender),
            activity_level: Set(activity_level),
            updated_at: NotSet,
        };
        user_infos.insert(&self.db).await
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<user_additional_infos::Model>, sea_orm::DbErr> {
        user_additional_infos::Entity::find_by_id(*user_id)
            .one(&self.db)
            .await
    }

    pub async fn update(
        &self,
        user_id: &Uuid,
        birth_date: Option<&chrono::NaiveDate>,
        height_in_cm: Option<i32>,
        gender: Option<GenderEnum>,
        activity_level: Option<Decimal>,
    ) -> Result<user_additional_infos::Model, sea_orm::DbErr> {
        if birth_date.is_none()
            && height_in_cm.is_none()
            && gender.is_none()
            && activity_level.is_none()
        {
            return Err(sea_orm::DbErr::Custom(
                "At least one field must be provided for update".to_string(),
            ));
        }

        let mut active = user_additional_infos::ActiveModel {
            user_id: Set(*user_id),
            ..Default::default()
        };

        if let Some(bd) = birth_date {
            active.birth_date = Set(*bd);
        }

        if let Some(h) = height_in_cm {
            active.height_in_cm = Set(h);
        }

        if let Some(g) = gender {
            active.gender = Set(g);
        }

        if let Some(a) = activity_level {
            active.activity_level = Set(a);
        }

        active.update(&self.db).await
    }
}
