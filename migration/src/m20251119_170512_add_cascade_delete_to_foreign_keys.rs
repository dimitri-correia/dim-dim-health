use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // This migration ensures all foreign keys to users have ON DELETE CASCADE
        // Even if the database was created before CASCADE was added to the migrations
        
        let db = manager.get_connection();

        // Drop and recreate foreign keys with CASCADE for email_verification_tokens
        db.execute_unprepared(
            r#"
            ALTER TABLE email_verification_tokens
            DROP CONSTRAINT IF EXISTS fk_email_verification_tokens_user_id;
            
            ALTER TABLE email_verification_tokens
            ADD CONSTRAINT fk_email_verification_tokens_user_id
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for password_reset_tokens
        db.execute_unprepared(
            r#"
            ALTER TABLE password_reset_tokens
            DROP CONSTRAINT IF EXISTS fk_password_reset_tokens_user_id;
            
            ALTER TABLE password_reset_tokens
            ADD CONSTRAINT fk_password_reset_tokens_user_id
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for refresh_tokens
        db.execute_unprepared(
            r#"
            ALTER TABLE refresh_tokens
            DROP CONSTRAINT IF EXISTS fk_refresh_tokens_user_id;
            
            ALTER TABLE refresh_tokens
            ADD CONSTRAINT fk_refresh_tokens_user_id
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for user_additional_infos
        db.execute_unprepared(
            r#"
            ALTER TABLE user_additional_infos
            DROP CONSTRAINT IF EXISTS fk_user_additional_infos_user_id;
            
            ALTER TABLE user_additional_infos
            ADD CONSTRAINT fk_user_additional_infos_user_id
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for user_groups
        db.execute_unprepared(
            r#"
            ALTER TABLE user_groups
            DROP CONSTRAINT IF EXISTS fk_user_groups_user_id;
            
            ALTER TABLE user_groups
            ADD CONSTRAINT fk_user_groups_user_id
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for user_watch_permissions (user_watched_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE user_watch_permissions
            DROP CONSTRAINT IF EXISTS fk_user_watch_permissions_user_watched_id;
            
            ALTER TABLE user_watch_permissions
            ADD CONSTRAINT fk_user_watch_permissions_user_watched_id
            FOREIGN KEY (user_watched_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for user_watch_permissions (user_watching_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE user_watch_permissions
            DROP CONSTRAINT IF EXISTS fk_user_watch_permissions_user_watching_id;
            
            ALTER TABLE user_watch_permissions
            ADD CONSTRAINT fk_user_watch_permissions_user_watching_id
            FOREIGN KEY (user_watching_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for user_weight
        db.execute_unprepared(
            r#"
            ALTER TABLE user_weight
            DROP CONSTRAINT IF EXISTS fk_weight_user_id;
            
            ALTER TABLE user_weight
            ADD CONSTRAINT fk_weight_user_id
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for food_item
        db.execute_unprepared(
            r#"
            ALTER TABLE food_item
            DROP CONSTRAINT IF EXISTS fk_food_item_added_by;
            
            ALTER TABLE food_item
            ADD CONSTRAINT fk_food_item_added_by
            FOREIGN KEY (added_by) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for meal
        db.execute_unprepared(
            r#"
            ALTER TABLE meal
            DROP CONSTRAINT IF EXISTS fk_meal_user_id;
            
            ALTER TABLE meal
            ADD CONSTRAINT fk_meal_user_id
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for meal_item (meal_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE meal_item
            DROP CONSTRAINT IF EXISTS fk_meal_item_meal_id;
            
            ALTER TABLE meal_item
            ADD CONSTRAINT fk_meal_item_meal_id
            FOREIGN KEY (meal_id) REFERENCES meal(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        // Drop and recreate foreign keys with CASCADE for meal_item (food_item_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE meal_item
            DROP CONSTRAINT IF EXISTS fk_meal_item_food_item_id;
            
            ALTER TABLE meal_item
            ADD CONSTRAINT fk_meal_item_food_item_id
            FOREIGN KEY (food_item_id) REFERENCES food_item(id) ON DELETE CASCADE;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // This down migration removes CASCADE from all foreign keys
        // Note: In a real production scenario, you'd want to be careful about this
        // as it might leave orphaned records
        
        let db = manager.get_connection();

        // Revert email_verification_tokens
        db.execute_unprepared(
            r#"
            ALTER TABLE email_verification_tokens
            DROP CONSTRAINT IF EXISTS fk_email_verification_tokens_user_id;
            
            ALTER TABLE email_verification_tokens
            ADD CONSTRAINT fk_email_verification_tokens_user_id
            FOREIGN KEY (user_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert password_reset_tokens
        db.execute_unprepared(
            r#"
            ALTER TABLE password_reset_tokens
            DROP CONSTRAINT IF EXISTS fk_password_reset_tokens_user_id;
            
            ALTER TABLE password_reset_tokens
            ADD CONSTRAINT fk_password_reset_tokens_user_id
            FOREIGN KEY (user_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert refresh_tokens
        db.execute_unprepared(
            r#"
            ALTER TABLE refresh_tokens
            DROP CONSTRAINT IF EXISTS fk_refresh_tokens_user_id;
            
            ALTER TABLE refresh_tokens
            ADD CONSTRAINT fk_refresh_tokens_user_id
            FOREIGN KEY (user_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert user_additional_infos
        db.execute_unprepared(
            r#"
            ALTER TABLE user_additional_infos
            DROP CONSTRAINT IF EXISTS fk_user_additional_infos_user_id;
            
            ALTER TABLE user_additional_infos
            ADD CONSTRAINT fk_user_additional_infos_user_id
            FOREIGN KEY (user_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert user_groups
        db.execute_unprepared(
            r#"
            ALTER TABLE user_groups
            DROP CONSTRAINT IF EXISTS fk_user_groups_user_id;
            
            ALTER TABLE user_groups
            ADD CONSTRAINT fk_user_groups_user_id
            FOREIGN KEY (user_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert user_watch_permissions (user_watched_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE user_watch_permissions
            DROP CONSTRAINT IF EXISTS fk_user_watch_permissions_user_watched_id;
            
            ALTER TABLE user_watch_permissions
            ADD CONSTRAINT fk_user_watch_permissions_user_watched_id
            FOREIGN KEY (user_watched_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert user_watch_permissions (user_watching_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE user_watch_permissions
            DROP CONSTRAINT IF EXISTS fk_user_watch_permissions_user_watching_id;
            
            ALTER TABLE user_watch_permissions
            ADD CONSTRAINT fk_user_watch_permissions_user_watching_id
            FOREIGN KEY (user_watching_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert user_weight
        db.execute_unprepared(
            r#"
            ALTER TABLE user_weight
            DROP CONSTRAINT IF EXISTS fk_weight_user_id;
            
            ALTER TABLE user_weight
            ADD CONSTRAINT fk_weight_user_id
            FOREIGN KEY (user_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert food_item
        db.execute_unprepared(
            r#"
            ALTER TABLE food_item
            DROP CONSTRAINT IF EXISTS fk_food_item_added_by;
            
            ALTER TABLE food_item
            ADD CONSTRAINT fk_food_item_added_by
            FOREIGN KEY (added_by) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert meal
        db.execute_unprepared(
            r#"
            ALTER TABLE meal
            DROP CONSTRAINT IF EXISTS fk_meal_user_id;
            
            ALTER TABLE meal
            ADD CONSTRAINT fk_meal_user_id
            FOREIGN KEY (user_id) REFERENCES users(id);
            "#,
        )
        .await?;

        // Revert meal_item (meal_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE meal_item
            DROP CONSTRAINT IF EXISTS fk_meal_item_meal_id;
            
            ALTER TABLE meal_item
            ADD CONSTRAINT fk_meal_item_meal_id
            FOREIGN KEY (meal_id) REFERENCES meal(id);
            "#,
        )
        .await?;

        // Revert meal_item (food_item_id)
        db.execute_unprepared(
            r#"
            ALTER TABLE meal_item
            DROP CONSTRAINT IF EXISTS fk_meal_item_food_item_id;
            
            ALTER TABLE meal_item
            ADD CONSTRAINT fk_meal_item_food_item_id
            FOREIGN KEY (food_item_id) REFERENCES food_item(id);
            "#,
        )
        .await?;

        Ok(())
    }
}
