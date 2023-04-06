use std::net::SocketAddr;

use server::{self, scheduler_grpc::scheduler_server::SchedulerServer};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:10000".parse().unwrap();

    let scheduler = server::SchedulerService {};

    let svc = SchedulerServer::new(scheduler);

    println!("Server listening on {}", addr.to_string());
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
