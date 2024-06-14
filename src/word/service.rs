use crate::entity::word;
use sea_orm::prelude::*;
pub struct Service {
    repository: super::repository::Repository,
    user_repository: crate::user::repository::Repository,
}

impl Service {
    pub fn new(
        repository: super::repository::Repository,
        user_repository: crate::user::repository::Repository,
    ) -> Self {
        Self {
            repository,
            user_repository,
        }
    }

    pub async fn find_word_by_id(&self, id: i32) -> Result<Option<word::Model>, DbErr> {
        self.repository.find_word_by_id(id).await
    }
}
