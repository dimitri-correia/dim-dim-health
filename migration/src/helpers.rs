use sea_orm_migration::prelude::*;

// Helper functions for creating updated_at triggers.
// The `update_updated_at_column` function is created once in the first migration (user_table)
// and reused by all subsequent migrations.

/// Creates a trigger for the `updated_at` column on a table.
/// Assumes the `update_updated_at_column` function already exists.
pub async fn create_updated_at_trigger(
    manager: &SchemaManager<'_>,
    table_name: &str,
) -> Result<(), DbErr> {
    let trigger_name = format!("update_{table_name}_updated_at");
    manager
        .get_connection()
        .execute_unprepared(&format!(
            r#"
            CREATE TRIGGER {trigger_name}
                BEFORE UPDATE ON {table_name}
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
            "#
        ))
        .await?;
    Ok(())
}

/// Drops a trigger for the `updated_at` column on a table.
pub async fn drop_updated_at_trigger(
    manager: &SchemaManager<'_>,
    table_name: &str,
) -> Result<(), DbErr> {
    let trigger_name = format!("update_{table_name}_updated_at");
    manager
        .get_connection()
        .execute_unprepared(&format!(
            "DROP TRIGGER IF EXISTS {trigger_name} ON {table_name};"
        ))
        .await?;
    Ok(())
}

// Helper functions for pg_cron scheduling.

/// Schedules a cron job using pg_cron.
/// Note: The `sql_command` will have single quotes escaped automatically.
pub async fn schedule_cron_job(
    manager: &SchemaManager<'_>,
    job_name: &str,
    schedule: &str,
    sql_command: &str,
) -> Result<(), DbErr> {
    // Escape single quotes for the SQL command
    let escaped_sql = sql_command.replace('\'', "''");

    manager
        .get_connection()
        .execute_unprepared(&format!(
            r#"
            SELECT cron.schedule(
                '{job_name}',
                '{schedule}',
                '{escaped_sql}'
            );
            "#
        ))
        .await?;
    Ok(())
}

/// Unschedules a cron job. Ignores errors if the job doesn't exist.
pub async fn unschedule_cron_job(manager: &SchemaManager<'_>, job_name: &str) -> Result<(), DbErr> {
    manager
        .get_connection()
        .execute_unprepared(&format!("SELECT cron.unschedule('{job_name}');"))
        .await
        .ok(); // Ignore errors if job doesn't exist
    Ok(())
}
