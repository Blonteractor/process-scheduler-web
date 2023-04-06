use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use server::{self, scheduler_grpc::scheduler_server::SchedulerServer};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        env::var("LISTEN_PORT").unwrap().parse().unwrap(),
    );

    let scheduler = server::SchedulerService {};

    let svc = SchedulerServer::new(scheduler);

    println!("Server listening on {}", addr.to_string());
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
