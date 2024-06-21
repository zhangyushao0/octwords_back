use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "learn_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub wrong_count: Option<i32>,
    pub user_id: i32,
    pub word_id: i32,
    pub envent_id: Option<Uuid>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity"
    from = "Column::UserId"
    to = "super::user::Column::Id")]
    User,
    #[sea_orm(belongs_to = "super::word::Entity"
    from = "Column::WordId"
    to = "super::word::Column::Id")]
    Word,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::word::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Word.def()
    }
}
