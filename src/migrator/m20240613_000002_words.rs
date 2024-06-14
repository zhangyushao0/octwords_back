use crate::entity::word;
use sea_orm::{
    sea_query::*, ActiveModelTrait, ActiveValue, Database, EntityName, EntityTrait, Schema,
};
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240613_000002_words"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        let st = builder.build(&schema.create_table_from_entity(word::Entity));
        manager.get_connection().execute(st).await?;

        let word_list = vec![
            "apple",
            "banana",
            "cherry",
            "date",
            "elderberry",
            "fig",
            "grape",
            "honeydew",
            "imbe",
            "jackfruit",
            "kiwi",
            "lemon",
            "mango",
            "nectarine",
            "orange",
            "papaya",
            "quince",
            "raspberry",
            "strawberry",
            "tangerine",
            "ugli",
            "vanilla",
            "watermelon",
            "ximenia",
            "yellow",
            "zucchini",
        ];

        let model_vec = word_list
            .iter()
            .map(|word| word::ActiveModel {
                word: ActiveValue::Set(word.to_string()),
                ..Default::default()
            })
            .collect::<Vec<word::ActiveModel>>();

        word::Entity::insert_many(model_vec)
            .exec(manager.get_connection())
            .await?;
        Ok(())
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(word::Entity).to_owned())
            .await
    }
}
