use rpc::run_rpc_server;
use std::sync::Arc;

mod rpc;

fn main() -> std::io::Result<()> {
    rpc::run_rpc_server();
    Ok(())
}
