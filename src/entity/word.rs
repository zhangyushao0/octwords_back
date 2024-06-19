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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
