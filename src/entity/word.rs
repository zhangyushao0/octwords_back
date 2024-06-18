use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "words")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i32,
    #[sea_orm(unique)]
    pub word: String,
    pub definition: Option<String>,
    pub translation: Option<String>,
    pub tag: Option<String>,
    pub extended_blocks: Option<Json>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::learn_event::Entity")]
    LearnEvent,
}

impl Related<super::learn_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LearnEvent.def()
    }
}
