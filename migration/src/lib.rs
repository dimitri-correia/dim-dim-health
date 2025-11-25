pub use sea_orm_migration::prelude::*;

mod m20251028_170454_user_table;
mod m20251101_123625_create_email_verification_tokens;
mod m20251101_124651_add_email_verified_to_users;
mod m20251105_173226_pg_cron_delete_expired_token;
mod m20251114_112914_create_password_reset_tokens;
mod m20251114_222407_create_refresh_tokens;
mod m20251114_234540_pg_cron_delete_expired_token;
mod m20251115_143529_create_user_info;
mod m20251115_151418_create_user_groups;
mod m20251115_153356_create_user_watch_permissions;
mod m20251115_154743_create_weight_table;
mod m20251115_160046_create_food_item;
mod m20251115_160750_create_meal;
mod m20251115_161449_create_meal_item;
mod m20251119_163331_pg_cron_clean_guest_users;
mod m20251119_172747_pg_cron_clean_non_verified_email;
mod m20251121_080000_create_email_preferences;
mod m20251121_080050_create_monthly_recap_queue;
mod m20251121_080100_pg_cron_monthly_recap;
mod m20251121_080150_create_weekly_recap_queue;
mod m20251121_080200_pg_cron_weekly_recap;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251028_170454_user_table::Migration),
            Box::new(m20251101_123625_create_email_verification_tokens::Migration),
            Box::new(m20251101_124651_add_email_verified_to_users::Migration),
            Box::new(m20251105_173226_pg_cron_delete_expired_token::Migration),
            Box::new(m20251114_112914_create_password_reset_tokens::Migration),
            Box::new(m20251114_222407_create_refresh_tokens::Migration),
            Box::new(m20251114_234540_pg_cron_delete_expired_token::Migration),
            Box::new(m20251115_143529_create_user_info::Migration),
            Box::new(m20251115_151418_create_user_groups::Migration),
            Box::new(m20251115_153356_create_user_watch_permissions::Migration),
            Box::new(m20251115_154743_create_weight_table::Migration),
            Box::new(m20251115_160046_create_food_item::Migration),
            Box::new(m20251115_160750_create_meal::Migration),
            Box::new(m20251115_161449_create_meal_item::Migration),
            Box::new(m20251119_163331_pg_cron_clean_guest_users::Migration),
            Box::new(m20251119_172747_pg_cron_clean_non_verified_email::Migration),
            Box::new(m20251121_080000_create_email_preferences::Migration),
            Box::new(m20251121_080050_create_monthly_recap_queue::Migration),
            Box::new(m20251121_080100_pg_cron_monthly_recap::Migration),
            Box::new(m20251121_080150_create_weekly_recap_queue::Migration),
            Box::new(m20251121_080200_pg_cron_weekly_recap::Migration),
        ]
    }
}
