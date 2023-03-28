use server::{self, scheduler_grpc::scheduler_server::SchedulerServer};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:10000".parse().unwrap();

    let scheduler = server::SchedulerService {};

    let svc = SchedulerServer::new(scheduler);

    println!("Server listening on localhost:10000");
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
