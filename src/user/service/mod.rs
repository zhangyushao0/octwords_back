pub mod jwt;
pub mod password;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::error::DbErr),
    #[error("Password excution error: {0}")]
    PasswordExecutionError(#[from] argon2::password_hash::Error),
    #[error("Cannot find user: {0}")]
    UserNotFound(String),
}

pub struct Service {
    repository: super::repository::Repository,
}

impl Service {
    pub fn new(repository: super::repository::Repository) -> Self {
        Self { repository }
    }

    pub async fn verify_user(&self, name: &str, password: &str) -> Result<bool, Error> {
        let user = self.repository.find_user_by_name(name).await?;
        match user {
            Some(user) => {
                let hash = user.hash_password;
                match Self::verify_password(password, &hash) {
                    Ok(_) => Ok(true),
                    Err(argon2::password_hash::Error::Password) => Ok(false),
                    Err(e) => Err(Error::PasswordExecutionError(e)),
                }
            }
            None => Err(Error::UserNotFound(name.to_owned())),
        }
    }
}
