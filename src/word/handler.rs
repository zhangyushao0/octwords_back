pub mod word {
    tonic::include_proto!("word");
}

use tonic::{transport::Server, Request, Response, Status};
use word::word_server::Word;
use word::{WordReply, WordRequest};

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
                word: word.word,
                definition: word.definition.unwrap_or_default(),
                translation: word.translation.unwrap_or_default(),
                tag: word
                    .tag
                    .unwrap_or_default()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            })),
            Ok(None) => Ok(Response::new(WordReply {
                word: "".to_string(),
                definition: "".to_string(),
                translation: "".to_string(),
                tag: vec![],
            })),
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
                req.extensions_mut().insert(sub);
                Ok(req)
            }
            Err(_) => Err(Status::unauthenticated("Invalid token")),
        }
    }
}
