use crate::helpers::{create_updated_at_trigger, drop_updated_at_trigger};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static MUSCLE_ENUM: &str = "muscle_enum";
static GYM_EXERCISE_TABLE_NAME: &str = "gym_exercise";
static MUSCLE_ROLE_ENUM: &str = "muscle_role_enum";
static EXERCISE_MUSCLE_TABLE_NAME: &str = "exercise_muscle";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create muscle enum
        manager
            .get_connection()
            .execute_unprepared(&format!(
                "CREATE TYPE {} AS ENUM (
                    'chest',
                    'back',
                    'shoulders',
                    'biceps',
                    'triceps',
                    'forearms',
                    'quadriceps',
                    'hamstrings',
                    'glutes',
                    'calves',
                    'abs',
                    'obliques',
                    'traps',
                    'lats',
                    'lower_back'
                );",
                MUSCLE_ENUM
            ))
            .await?;

        // Create muscle role enum
        manager
            .get_connection()
            .execute_unprepared(&format!(
                "CREATE TYPE {} AS ENUM (
                    'primary',
                    'secondary'
                );",
                MUSCLE_ROLE_ENUM
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GymExercise::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GymExercise::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(GymExercise::Name).string_len(255).not_null())
                    .col(ColumnDef::new(GymExercise::Description).text().null())
                    .col(ColumnDef::new(GymExercise::AddedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(GymExercise::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(GymExercise::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_exercise_added_by")
                            .from(GymExercise::Table, GymExercise::AddedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ExerciseMuscle::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExerciseMuscle::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(ExerciseMuscle::ExerciseId).uuid().not_null())
                    .col(
                        ColumnDef::new(ExerciseMuscle::Muscle)
                            .custom(Alias::new("muscle_enum"))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExerciseMuscle::Role)
                            .custom(Alias::new(MUSCLE_ROLE_ENUM))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExerciseMuscle::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ExerciseMuscle::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exercise_muscle_exercise_id")
                            .from(ExerciseMuscle::Table, ExerciseMuscle::ExerciseId)
                            .to(GymExercise::Table, GymExercise::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_gym_exercise_added_by")
                    .table(GymExercise::Table)
                    .col(GymExercise::AddedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_gym_exercise_name")
                    .table(GymExercise::Table)
                    .col(GymExercise::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_exercise_muscle_exercise_id")
                    .table(ExerciseMuscle::Table)
                    .col(ExerciseMuscle::ExerciseId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_exercise_muscle_muscle")
                    .table(ExerciseMuscle::Table)
                    .col(ExerciseMuscle::Muscle)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_exercise_muscle_role")
                    .table(ExerciseMuscle::Table)
                    .col(ExerciseMuscle::Role)
                    .to_owned(),
            )
            .await?;

        // Create unique constraint to prevent duplicate muscle-exercise combinations whatever the role
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("idx_exercise_muscle_unique")
                    .table(ExerciseMuscle::Table)
                    .col(ExerciseMuscle::ExerciseId)
                    .col(ExerciseMuscle::Muscle)
                    .to_owned(),
            )
            .await?;

        // Add trigger for updated_at
        create_updated_at_trigger(manager, GYM_EXERCISE_TABLE_NAME).await?;
        create_updated_at_trigger(manager, EXERCISE_MUSCLE_TABLE_NAME).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop trigger
        drop_updated_at_trigger(manager, GYM_EXERCISE_TABLE_NAME).await?;
        drop_updated_at_trigger(manager, EXERCISE_MUSCLE_TABLE_NAME).await?;

        manager
            .drop_table(Table::drop().table(ExerciseMuscle::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GymExercise::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", MUSCLE_ENUM))
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&format!("DROP TYPE IF EXISTS {};", MUSCLE_ROLE_ENUM))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ExerciseMuscle {
    Table,
    Id,
    ExerciseId,
    Muscle,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum GymExercise {
    Table,
    Id,
    Name,
    Description,
    AddedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
