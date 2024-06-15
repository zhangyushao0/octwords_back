mod db_setup;
mod entity;
mod migrator;
mod user;
mod word;

use crate::db_setup::set_up_db;
use tonic::transport::Server;
use user::handler::login::login_server::LoginServer;
use word::handler::word::word_server::WordServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => {
            panic!("Error setting up db: {:?}", err);
        }
    };

    let addr = "0.0.0.0:50051".parse().unwrap();
    let login_handler = user::init(db.clone());

    let login_service = LoginServer::new(login_handler);

    let word_handler = word::init(db.clone());
    let new_db = db.clone();
    let word_service_with_auth = WordServer::with_interceptor(word_handler, move |req| {
        word::init(new_db.clone()).auth_intercept(req)
    });
    Server::builder()
        .add_service(login_service)
        .add_service(word_service_with_auth)
        .serve(addr)
        .await?;

    Ok(())
}
