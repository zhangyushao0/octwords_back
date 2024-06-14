use crate::entity::*;
use crate::migrator::Migrator;
use sea_orm::prelude::*;
use sea_orm::{sea_query::*, Database, EntityName, Schema};
use sea_orm_migration::{MigratorTrait, SchemaManager};

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url).await?;

    Migrator::refresh(&db).await?;

    Ok(db)
}
