pub use sea_orm_migration::prelude::*;

mod m20240613_000001_users;
mod m20240613_000002_words;
mod m20240620_000003_learn_event;
mod password;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240613_000001_users::Migration),
            Box::new(m20240613_000002_words::Migration),
            Box::new(m20240620_000003_learn_event::Migration),
        ]
    }
}
