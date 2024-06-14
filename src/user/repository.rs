use crate::entity::user;
use sea_orm::entity::prelude::*;
pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn find_user_by_name(&self, name: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Name.contains(name))
            .one(&self.db)
            .await
    }
}
