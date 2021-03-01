use std::sync::Arc;
use tonic::transport::{Error, Server};

mod sessions;

/// Creates and runs a new Rpc server with our custom app state and procedures
pub async fn run_rpc_server() -> Result<(), Error> {
    let address = format!("{}:{}", "127.0.0.1", "1337");

    Server::builder()
        .add_service(sessions::SessionsRpc::new())
        .serve(address.parse().unwrap())
        .await
}
