pub use sea_orm_migration::prelude::*;

pub mod helpers;

mod m20251028_170454_user_table;
mod m20251101_123625_create_email_verification_tokens;
mod m20251114_112914_create_password_reset_tokens;
mod m20251114_222407_create_refresh_tokens;
mod m20251115_143529_create_user_info;
mod m20251115_151418_create_user_groups;
mod m20251115_153356_create_user_watch_permissions;
mod m20251115_154743_create_weight_table;
mod m20251115_160046_create_food_item;
mod m20251115_160750_create_meal;
mod m20251115_161449_create_meal_item;
mod m20251121_080000_create_email_preferences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251028_170454_user_table::Migration),
            Box::new(m20251101_123625_create_email_verification_tokens::Migration),
            Box::new(m20251114_112914_create_password_reset_tokens::Migration),
            Box::new(m20251114_222407_create_refresh_tokens::Migration),
            Box::new(m20251115_143529_create_user_info::Migration),
            Box::new(m20251115_151418_create_user_groups::Migration),
            Box::new(m20251115_153356_create_user_watch_permissions::Migration),
            Box::new(m20251115_154743_create_weight_table::Migration),
            Box::new(m20251115_160046_create_food_item::Migration),
            Box::new(m20251115_160750_create_meal::Migration),
            Box::new(m20251115_161449_create_meal_item::Migration),
            Box::new(m20251121_080000_create_email_preferences::Migration),
        ]
    }
}
