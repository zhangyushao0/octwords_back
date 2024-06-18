pub mod jwt;
pub mod password;
use crate::entity::user;
use sea_orm::entity::prelude::*;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::error::DbErr),
    #[error("Password excution error: {0}")]
    PasswordExecutionError(#[from] argon2::password_hash::Error),
    #[error("Cannot find user: {0}")]
    UserNotFound(String),
    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

pub struct Service {
    db: DatabaseConnection,
}

impl Service {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn verify_user(&self, name: &str, password: &str) -> Result<bool, Error> {
        let user = user::Entity::find_by_name(name).one(&self.db).await?;
        match user {
            Some(user) => {
                let hash = user.hash_password;
                let password = password.to_owned();
                let is_valid =
                    tokio::task::spawn_blocking(move || Self::verify_password(&password, &hash))
                        .await?;
                match is_valid {
                    Ok(_) => Ok(true),
                    Err(argon2::password_hash::Error::Password) => Ok(false),
                    Err(e) => Err(Error::PasswordExecutionError(e)),
                }
            }
            None => Err(Error::UserNotFound(name.to_owned())),
        }
    }
}
