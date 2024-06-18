pub mod handler;
pub mod service;

use sea_orm::DatabaseConnection;

pub fn init(connection: DatabaseConnection) -> handler::Handler {
    let service = service::Service::new(connection);
    handler::Handler::new(service)
}
