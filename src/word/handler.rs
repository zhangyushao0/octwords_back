pub mod word {
    tonic::include_proto!("word");
}

use tonic::{transport::Server, Request, Response, Status};
use word::word_server::Word;
use word::{
    FinishLearnReply, FinishLearnRequest, NewLearnReply, NewLearnRequest, WordInner,
    WordLearnInner, WordReply, WordRequest,
};

struct UserIdExtension {
    user_id: i32,
}

pub struct Handler {
    service: super::service::Service,
}

#[tonic::async_trait]
impl Word for Handler {
    async fn get_word(&self, request: Request<WordRequest>) -> Result<Response<WordReply>, Status> {
        let req = request.into_inner();
        let id = req.id;
        match self.service.find_word_by_id(id as i32).await {
            Ok(Some(word)) => Ok(Response::new(WordReply {
                word: Some(WordInner {
                    id: word.id as u32,
                    word: word.word,
                    definition: word.definition.unwrap_or_default(),
                    translation: word.translation.unwrap_or_default(),
                    tag: word
                        .tag
                        .unwrap_or_default()
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect(),
                    extended_blocks: word.extended_blocks.unwrap_or_default().to_string(),
                }),
            })),
            Ok(None) => Ok(Response::new(WordReply::default())),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
    async fn start_new_learn(
        &self,
        request: Request<NewLearnRequest>,
    ) -> Result<Response<NewLearnReply>, Status> {
        let user_id = request
            .extensions()
            .get::<UserIdExtension>()
            .unwrap()
            .user_id;
        let req = request.into_inner();
        let word_num = req.number;
        match self
            .service
            .start_new_learn_event(user_id, word_num as i32)
            .await
        {
            Ok((words, event_id)) => {
                let word_list = words
                    .iter()
                    .map(|word| WordInner {
                        id: word.id as u32,
                        word: word.word.clone(),
                        definition: word.definition.clone().unwrap_or_default(),
                        translation: word.translation.clone().unwrap_or_default(),
                        tag: word
                            .tag
                            .clone()
                            .unwrap_or_default()
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect(),
                        extended_blocks: word
                            .extended_blocks
                            .clone()
                            .unwrap_or_default()
                            .to_string(),
                    })
                    .collect();
                Ok(Response::new(NewLearnReply {
                    words: word_list,
                    event_id: event_id.to_string(),
                }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn finish_learn(
        &self,
        request: Request<FinishLearnRequest>,
    ) -> Result<Response<FinishLearnReply>, Status> {
        let user_id = request
            .extensions()
            .get::<UserIdExtension>()
            .unwrap()
            .user_id;
        let req = request.into_inner();
        let event_id = req.event_id.parse().unwrap();
        let learn_events_return = req
            .words
            .into_iter()
            .map(|learn_event| (learn_event.id as i32, learn_event.wrong_count as i32))
            .collect();

        match self
            .service
            .finish_learn_event(user_id, event_id, learn_events_return)
            .await
        {
            Ok(_) => Ok(Response::new(FinishLearnReply::default())),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

impl Handler {
    pub fn new(service: super::service::Service) -> Self {
        Self { service }
    }
    pub fn auth_intercept(&self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let metadata = req.metadata_mut();
        let token = match metadata.get("authorization") {
            Some(token) => token.to_str().unwrap(),
            None => return Err(Status::unauthenticated("No token provided")),
        };
        let verified = crate::user::service::Service::verify_token(token);
        match verified {
            Ok(sub) => {
                req.extensions_mut().insert(UserIdExtension {
                    user_id: sub.parse().unwrap(),
                });
                Ok(req)
            }
            Err(_) => Err(Status::unauthenticated("Invalid token")),
        }
    }
}
