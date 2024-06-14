pub mod handler;
pub mod repository;
pub mod service;

use sea_orm::DatabaseConnection;

pub fn init(connection: DatabaseConnection) -> handler::Handler {
    let repository = repository::Repository::new(connection.clone());
    let user_repository = crate::user::repository::Repository::new(connection.clone());

    let service = service::Service::new(repository, user_repository);
    handler::Handler::new(service)
}
