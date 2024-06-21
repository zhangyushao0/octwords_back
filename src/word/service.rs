use crate::entity::learn_event;
use crate::entity::user;
use crate::entity::word;
use sea_orm::prelude::*;
use sea_orm::ActiveValue;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::QueryTrait;
pub struct Service {
    db: DatabaseConnection,
}

impl Service {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_word_by_id(&self, id: i32) -> Result<Option<word::Model>, DbErr> {
        word::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn start_new_learn_event(
        &self,
        user_id: i32,
        word_num: i32,
    ) -> Result<(Vec<word::Model>, Uuid), DbErr> {
        let the_user = user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .unwrap();
        let subquery = learn_event::Entity::find()
            .filter(learn_event::Column::UserId.eq(user_id))
            .select_only()
            .column(learn_event::Column::WordId)
            .into_query();

        let words = word::Entity::find()
            .order_by_asc(word::Column::Id)
            .filter(word::Column::Id.not_in_subquery(subquery))
            .limit(word_num as u64)
            .all(&self.db)
            .await?;

        let mut new_learn_events = vec![];
        let envet_id = Uuid::new_v4();
        for word in words.iter() {
            let new_learn_event = learn_event::ActiveModel {
                user_id: ActiveValue::Set(the_user.id.clone()),
                word_id: ActiveValue::Set(word.id.clone()),
                start_time: ActiveValue::Set(Some(chrono::Local::now().naive_local())),
                envent_id: ActiveValue::Set(Some(envet_id)),
                ..Default::default()
            };
            new_learn_events.push(new_learn_event);
        }

        learn_event::Entity::insert_many(new_learn_events)
            .exec(&self.db)
            .await?;

        Ok((words, envet_id))
    }

    pub async fn finish_learn_event(
        &self,
        user_id: i32,
        event_id: Uuid,
        learn_events_return: Vec<(i32, i32)>, // (word_id, wrong_count)
    ) -> Result<(), DbErr> {
        let learn_events = learn_event::Entity::find()
            .filter(learn_event::Column::UserId.eq(user_id))
            .filter(learn_event::Column::EnventId.eq(Some(event_id)))
            .all(&self.db)
            .await?;

        let end_time = chrono::Local::now().naive_local();
        for learn_event in learn_events {
            let mut new_learn_event: learn_event::ActiveModel = learn_event.into();
            new_learn_event.end_time = ActiveValue::Set(Some(end_time));
            new_learn_event.wrong_count = ActiveValue::Set(Some(
                learn_events_return
                    .iter()
                    .find(|(word_id, _)| *word_id == new_learn_event.word_id.clone().unwrap())
                    .unwrap()
                    .1,
            ));
            new_learn_event.update(&self.db).await?;
        }
        Ok(())
    }
}
