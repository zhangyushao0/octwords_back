pub mod login {
    tonic::include_proto!("login");
}

use login::login_server::Login;
use login::{LoginReply, LoginRequest};
use tonic::{transport::Server, Request, Response, Status};

pub struct Handler {
    service: super::service::Service,
}

#[tonic::async_trait]
impl Login for Handler {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        let req = request.into_inner();
        let username = req.username;
        let password = req.password;
        match self.service.verify_user(&username, &password).await {
            Ok(true) => {
                let user_id = self
                    .service
                    .get_user_id(&username)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                Ok(Response::new(LoginReply {
                    success: true,
                    token: self.service.create_token(user_id).unwrap(),
                }))
            }
            Ok(false) => Ok(Response::new(LoginReply {
                success: false,
                token: "".to_string(),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

impl Handler {
    pub fn new(service: super::service::Service) -> Self {
        Self { service }
    }
}
