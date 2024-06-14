pub mod handler;
pub mod repository;
pub mod service;

use sea_orm::DatabaseConnection;

pub fn init(connection: DatabaseConnection) -> handler::Handler {
    let repository = repository::Repository::new(connection);
    let service = service::Service::new(repository);
    handler::Handler::new(service)
}
