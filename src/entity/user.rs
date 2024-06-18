use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub hash_password: String,
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_name(name: &str) -> Select<Entity> {
        Self::find().filter(Column::Name.contains(name))
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::learn_event::Entity")]
    LearnEvent,
}

impl Related<super::learn_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LearnEvent.def()
    }
}
