use crate::entity::word;
use sea_orm::entity::prelude::*;
pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_word_by_id(&self, id: i32) -> Result<Option<word::Model>, DbErr> {
        word::Entity::find_by_id(id).one(&self.db).await
    }
}
