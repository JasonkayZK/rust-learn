use dotenv::dotenv;
use log::info;
use rust_learn::api::StorageService;
use std::env;
use tonic::transport::Server;

use rust_learn::logger;
use rust_learn::storage_proto::storage_server::StorageServer;
use rust_learn::utils::get_port;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    logger::init();
    let server_port = get_port(env::args().collect());
    let addr = format!("0.0.0.0:{}", server_port);
    info!("Listening on port {}", addr);

    Server::builder()
        .add_service(StorageServer::new(StorageService::default()))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
