use crate::helpers::{create_updated_at_trigger, drop_updated_at_trigger};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

static TABLE_NAME: &str = "gym_set";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GymSet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GymSet::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(GymSet::SessionId).uuid().not_null())
                    .col(ColumnDef::new(GymSet::ExerciseId).uuid().not_null())
                    .col(
                        ColumnDef::new(GymSet::SetNumber)
                            .integer()
                            .not_null()
                            .check(Expr::col(GymSet::SetNumber).gte(Expr::value(1))),
                    )
                    .col(
                        ColumnDef::new(GymSet::Repetitions)
                            .integer()
                            .not_null()
                            .check(Expr::col(GymSet::Repetitions).gte(Expr::value(0))),
                    )
                    .col(
                        ColumnDef::new(GymSet::WeightKg)
                            .decimal_len(6, 2)
                            .not_null()
                            .check(Expr::col(GymSet::WeightKg).gte(Expr::value(0.0))),
                    )
                    .col(ColumnDef::new(GymSet::Notes).text().null())
                    .col(
                        ColumnDef::new(GymSet::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(GymSet::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_set_session_id")
                            .from(GymSet::Table, GymSet::SessionId)
                            .to(GymSession::Table, GymSession::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_set_exercise_id")
                            .from(GymSet::Table, GymSet::ExerciseId)
                            .to(GymExercise::Table, GymExercise::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_gym_set_session_id")
                    .table(GymSet::Table)
                    .col(GymSet::SessionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_gym_set_exercise_id")
                    .table(GymSet::Table)
                    .col(GymSet::ExerciseId)
                    .to_owned(),
            )
            .await?;

        // Add trigger for updated_at
        create_updated_at_trigger(manager, TABLE_NAME).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop trigger
        drop_updated_at_trigger(manager, TABLE_NAME).await?;

        manager
            .drop_table(Table::drop().table(GymSet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GymSet {
    Table,
    Id,
    SessionId,
    ExerciseId,
    SetNumber,
    Repetitions,
    WeightKg,
    Notes,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum GymSession {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum GymExercise {
    Table,
    Id,
}
