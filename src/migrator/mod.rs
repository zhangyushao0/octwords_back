mod m20240613_000001_users;
mod m20240613_000002_words;
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240613_000001_users::Migration),
            Box::new(m20240613_000002_words::Migration),
        ]
    }
}
