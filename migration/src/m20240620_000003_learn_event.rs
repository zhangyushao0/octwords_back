use octwords_back_entity::learn_event;
use sea_orm::{
    sea_query::*, ActiveModelTrait, ActiveValue, Database, EntityName, EntityTrait, Schema,
};
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240620_000003_learn_event"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        let st = builder.build(&schema.create_table_from_entity(learn_event::Entity));
        manager.get_connection().execute(st).await?;
        Ok(())
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(learn_event::Entity).to_owned())
            .await
    }
}
