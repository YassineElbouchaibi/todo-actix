use entity::todo::{Column as TodoColumn, Entity as TodoEntity};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TodoEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TodoColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TodoColumn::Title).string().not_null())
                    .col(
                        ColumnDef::new(TodoColumn::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TodoColumn::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TodoColumn::CompletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE todos ALTER COLUMN created_at SET DEFAULT timezone('utc', now())"
                    .to_owned(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE todos ALTER COLUMN updated_at SET DEFAULT timezone('utc', now())"
                    .to_owned(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoEntity).to_owned())
            .await
    }
}
