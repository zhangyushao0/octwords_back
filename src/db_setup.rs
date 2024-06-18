use sea_orm::prelude::*;
use sea_orm::{sea_query::*, Database, EntityName, Schema};

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url).await?;

    Ok(db)
}
