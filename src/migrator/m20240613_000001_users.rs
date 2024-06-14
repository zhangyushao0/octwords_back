use crate::entity::user;
use sea_orm::{
    sea_query::*, ActiveModelTrait, ActiveValue, Database, EntityName, EntityTrait, Schema,
};
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240613_000001_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        let st = builder.build(&schema.create_table_from_entity(user::Entity));
        manager.get_connection().execute(st).await?;
        let username = "zhangyushao";
        let hash_password = crate::user::service::Service::hash_password("ling0017").unwrap();
        let new_user = user::ActiveModel {
            name: ActiveValue::Set(username.to_owned()),
            hash_password: ActiveValue::Set(hash_password),
            ..Default::default()
        };
        let res = user::Entity::insert(new_user)
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}
