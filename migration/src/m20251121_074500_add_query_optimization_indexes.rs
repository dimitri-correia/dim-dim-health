use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Composite index for email_verification_tokens (token, expires_at)
        // Optimizes: find_by_token which filters by token AND expires_at
        manager
            .create_index(
                Index::create()
                    .name("idx_email_verification_tokens_token_expires_at")
                    .table(EmailVerificationToken::Table)
                    .col(EmailVerificationToken::Token)
                    .col(EmailVerificationToken::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        // Composite index for password_reset_tokens (token, expires_at)
        // Optimizes: find_by_token which filters by token AND expires_at
        manager
            .create_index(
                Index::create()
                    .name("idx_password_reset_tokens_token_expires_at")
                    .table(PasswordResetToken::Table)
                    .col(PasswordResetToken::Token)
                    .col(PasswordResetToken::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        // Index on user_weight.recorded_at
        // Optimizes: time-series queries for weight trends and historical data
        manager
            .create_index(
                Index::create()
                    .name("idx_user_weight_recorded_at")
                    .table(UserWeight::Table)
                    .col(UserWeight::RecordedAt)
                    .to_owned(),
            )
            .await?;

        // Composite index on user_weight (user_id, recorded_at)
        // Optimizes: common query pattern for user weight history ordered by date
        manager
            .create_index(
                Index::create()
                    .name("idx_user_weight_user_id_recorded_at")
                    .table(UserWeight::Table)
                    .col(UserWeight::UserId)
                    .col(UserWeight::RecordedAt)
                    .to_owned(),
            )
            .await?;

        // Index on meal.date
        // Optimizes: queries filtering meals by date across all users
        manager
            .create_index(
                Index::create()
                    .name("idx_meal_date")
                    .table(Meal::Table)
                    .col(Meal::Date)
                    .to_owned(),
            )
            .await?;

        // Composite index on meal (user_id, date)
        // Optimizes: very common query pattern for user meals on specific dates
        manager
            .create_index(
                Index::create()
                    .name("idx_meal_user_id_date")
                    .table(Meal::Table)
                    .col(Meal::UserId)
                    .col(Meal::Date)
                    .to_owned(),
            )
            .await?;

        // Index on meal_item.food_item_id
        // Optimizes: reverse lookups to find all meals containing a specific food item
        manager
            .create_index(
                Index::create()
                    .name("idx_meal_item_food_item_id")
                    .table(MealItem::Table)
                    .col(MealItem::FoodItemId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes in reverse order
        manager
            .drop_index(
                Index::drop()
                    .name("idx_meal_item_food_item_id")
                    .table(MealItem::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_meal_user_id_date")
                    .table(Meal::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_meal_date")
                    .table(Meal::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_weight_user_id_recorded_at")
                    .table(UserWeight::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_weight_recorded_at")
                    .table(UserWeight::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_password_reset_tokens_token_expires_at")
                    .table(PasswordResetToken::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_email_verification_tokens_token_expires_at")
                    .table(EmailVerificationToken::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EmailVerificationToken {
    Table,
    Token,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum PasswordResetToken {
    Table,
    Token,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum UserWeight {
    Table,
    UserId,
    RecordedAt,
}

#[derive(DeriveIden)]
enum Meal {
    Table,
    UserId,
    Date,
}

#[derive(DeriveIden)]
enum MealItem {
    Table,
    FoodItemId,
}
