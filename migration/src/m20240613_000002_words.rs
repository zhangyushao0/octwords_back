use octwords_back_entity::stardict;
use octwords_back_entity::word;
use sea_orm::prelude::*;
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

        let word_list = get_words_from_stardict().await?;
        // split the word_list into chunks of 1000
        let word_list_list = word_list.chunks(1000).collect::<Vec<_>>();
        for word_list in word_list_list {
            word::Entity::insert_many(word_list.to_vec())
                .exec(manager.get_connection())
                .await?;
        }
        Ok(())
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(word::Entity).to_owned())
            .await
    }
}

pub async fn connect_to_stardict() -> Result<DatabaseConnection, DbErr> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let database_url = format!(
        "sqlite://{}",
        current_dir
            .join("migration")
            .join("stardict.sqlite")
            .to_str()
            .unwrap()
    );

    let db = Database::connect(&database_url).await?;

    Ok(db)
}

pub async fn get_words_from_stardict() -> Result<Vec<word::ActiveModel>, DbErr> {
    let db = connect_to_stardict().await?;
    // tag LIKE '%toefl%'
    // OR tag LIKE '%gre%'
    // OR (bnc > '0' AND bnc < '6000')
    // OR (frq > '0' AND frq < '6000')
    // OR oxford = '1'
    // OR collins > '2'
    let filter = stardict::Column::Tag
        .contains("toefl")
        .or(stardict::Column::Tag.contains("gre"))
        .or(stardict::Column::Bnc
            .gt(0)
            .and(stardict::Column::Bnc.lt(6000)))
        .or(stardict::Column::Frq
            .gt(0)
            .and(stardict::Column::Frq.lt(6000)))
        .or(stardict::Column::Oxford.eq(1))
        .or(stardict::Column::Collins.gt(2));
    let words = stardict::Entity::find().filter(filter).all(&db).await?;

    let mut word_list = Vec::new();

    for word in words {
        let mut tag = word.tag;
        // add oxford to tag
        if word.oxford.unwrap_or_default() == 1 {
            tag = Some(format!("{} oxford", tag.clone().unwrap_or_default()));
        }
        // add collins to tag
        if word.collins.unwrap_or_default() > 2 {
            tag = Some(format!(
                "{} collins{}",
                tag.clone().unwrap_or_default(),
                word.collins.unwrap_or_default()
            ));
        }
        word_list.push(word::ActiveModel {
            word: ActiveValue::Set(word.word),
            definition: ActiveValue::Set(word.definition),
            translation: ActiveValue::Set(word.translation),
            tag: ActiveValue::Set(tag),
            ..Default::default()
        });
    }

    Ok(word_list)
}
