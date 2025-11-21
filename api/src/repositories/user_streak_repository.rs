use chrono::{Datelike, Days, NaiveDate, Utc};
use entities::sea_orm_active_enums::SubAppTypeEnum;
use entities::{meal, user_streaks, user_weight};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserStreakRepository {
    db: DatabaseConnection,
}

impl UserStreakRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Get or create a streak record for a user and sub-app
    pub async fn get_or_create_streak(
        &self,
        user_id: &Uuid,
        sub_app: SubAppTypeEnum,
    ) -> Result<user_streaks::Model, sea_orm::DbErr> {
        // Try to find existing streak
        if let Some(streak) = user_streaks::Entity::find()
            .filter(user_streaks::Column::UserId.eq(user_id.to_owned()))
            .filter(user_streaks::Column::SubApp.eq(sub_app.clone()))
            .one(&self.db)
            .await?
        {
            return Ok(streak);
        }

        // Create new streak if not found
        let new_streak = user_streaks::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id.to_owned()),
            sub_app: Set(sub_app),
            current_streak: Set(0),
            longest_streak: Set(0),
            last_activity_week: Set(None),
            updated_at: Set(Utc::now().into()),
        };

        new_streak.insert(&self.db).await
    }

    /// Get all streaks for a user
    pub async fn get_user_streaks(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<user_streaks::Model>, sea_orm::DbErr> {
        user_streaks::Entity::find()
            .filter(user_streaks::Column::UserId.eq(user_id.to_owned()))
            .all(&self.db)
            .await
    }

    /// Calculate and update streak for weight sub-app
    pub async fn update_weight_streak(
        &self,
        user_id: &Uuid,
    ) -> Result<user_streaks::Model, sea_orm::DbErr> {
        // Get all weight entries for the user, ordered by date
        let weight_entries = user_weight::Entity::find()
            .filter(user_weight::Column::UserId.eq(user_id.to_owned()))
            .order_by_desc(user_weight::Column::RecordedAt)
            .all(&self.db)
            .await?;

        self.calculate_and_update_streak(user_id, SubAppTypeEnum::Weight, weight_entries)
            .await
    }

    /// Calculate and update streak for diet sub-app
    pub async fn update_diet_streak(
        &self,
        user_id: &Uuid,
    ) -> Result<user_streaks::Model, sea_orm::DbErr> {
        // Get all meal entries for the user, ordered by date
        let meal_entries = meal::Entity::find()
            .filter(meal::Column::UserId.eq(user_id.to_owned()))
            .order_by_desc(meal::Column::Date)
            .all(&self.db)
            .await?;

        self.calculate_and_update_streak(user_id, SubAppTypeEnum::Diet, meal_entries)
            .await
    }

    /// Calculate streak based on weekly activity
    async fn calculate_and_update_streak<T>(
        &self,
        user_id: &Uuid,
        sub_app: SubAppTypeEnum,
        entries: Vec<T>,
    ) -> Result<user_streaks::Model, sea_orm::DbErr>
    where
        T: HasDate,
    {
        let (current_streak, longest_streak, last_activity_week) =
            Self::calculate_streak_from_entries(entries);

        // Get or create streak record
        let streak = self.get_or_create_streak(user_id, sub_app.clone()).await?;

        // Update the streak
        let mut active_streak: user_streaks::ActiveModel = streak.into();
        active_streak.current_streak = Set(current_streak);
        active_streak.longest_streak = Set(longest_streak.max(active_streak.longest_streak.unwrap()));
        active_streak.last_activity_week = Set(last_activity_week);
        active_streak.updated_at = Set(Utc::now().into());

        active_streak.update(&self.db).await
    }

    /// Calculate streaks from a list of entries with dates
    fn calculate_streak_from_entries<T>(entries: Vec<T>) -> (i32, i32, Option<NaiveDate>)
    where
        T: HasDate,
    {
        if entries.is_empty() {
            return (0, 0, None);
        }

        // Group entries by week (Monday as start of week)
        let mut weeks_with_activity: Vec<NaiveDate> = entries
            .iter()
            .map(|entry| Self::get_week_start(entry.get_date()))
            .collect();

        // Remove duplicates and sort in descending order
        weeks_with_activity.sort_by(|a, b| b.cmp(a));
        weeks_with_activity.dedup();

        if weeks_with_activity.is_empty() {
            return (0, 0, None);
        }

        let last_activity_week = weeks_with_activity[0];
        let current_week_start = Self::get_week_start(Utc::now().date_naive());

        // Calculate current streak
        let mut current_streak = 0;
        let mut expected_week = current_week_start;

        for week in &weeks_with_activity {
            if *week == expected_week || *week == expected_week.checked_sub_days(Days::new(7)).unwrap() {
                current_streak += 1;
                expected_week = week.checked_sub_days(Days::new(7)).unwrap();
            } else {
                break;
            }
        }

        // Calculate longest streak
        let mut longest_streak = 0;
        let mut temp_streak = 1;

        for i in 1..weeks_with_activity.len() {
            let prev_week = weeks_with_activity[i - 1];
            let curr_week = weeks_with_activity[i];

            // Check if current week is exactly one week before previous week
            if curr_week == prev_week.checked_sub_days(Days::new(7)).unwrap() {
                temp_streak += 1;
            } else {
                longest_streak = longest_streak.max(temp_streak);
                temp_streak = 1;
            }
        }
        longest_streak = longest_streak.max(temp_streak);

        (current_streak, longest_streak, Some(last_activity_week))
    }

    /// Get the start of the week (Monday) for a given date
    fn get_week_start(date: NaiveDate) -> NaiveDate {
        let weekday = date.weekday().num_days_from_monday();
        date.checked_sub_days(Days::new(weekday as u64))
            .unwrap_or(date)
    }
}

/// Trait for types that have a date
trait HasDate {
    fn get_date(&self) -> NaiveDate;
}

impl HasDate for user_weight::Model {
    fn get_date(&self) -> NaiveDate {
        self.recorded_at.date_naive()
    }
}

impl HasDate for meal::Model {
    fn get_date(&self) -> NaiveDate {
        self.date
    }
}
