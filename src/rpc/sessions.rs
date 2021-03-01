use testing::sessions_server::{Sessions, SessionsServer};
use testing::{CreateSessionReply, CreateSessionRequest};
use tonic::{Request, Response, Status};

pub mod testing {
    tonic::include_proto!("sessions");
}

pub struct SessionsRpc;

impl SessionsRpc {
    pub fn new() -> SessionsServer<SessionsRpc> {
        SessionsServer::<SessionsRpc>::new(SessionsRpc)
    }
}

#[tonic::async_trait]
impl Sessions for SessionsRpc {
    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<CreateSessionReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let message = request.into_inner();
        println!("User info: {}:{}", message.user_id, message.client_ip);

        let reply = CreateSessionReply { message: "".to_string() };
        Ok(Response::new(reply))
    }
}
