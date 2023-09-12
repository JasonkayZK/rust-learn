use std::env;
use dotenv::dotenv;
use futures::{future, StreamExt};
use log::{info};
use tarpc::server::incoming::Incoming;
use tarpc::tokio_serde::formats::Json;


use tarpc::server;
use rust_learn::api::{Storage, StorageServer};
use rust_learn::logger;
use tarpc::server::Channel;
use rust_learn::utils::get_port;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    logger::init();
    let server_port = get_port(env::args().collect());

    let mut listener = tarpc::serde_transport::tcp::listen(format!("0.0.0.0:{}", server_port), Json::default).await?;
    info!("Listening on port {}", listener.local_addr().port());
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| future::ready(r.ok())) // Ignore accept errors.
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip()) // Limit channels to 1 per IP.
        .map(|channel| {
            channel.execute(StorageServer.serve())
        })
        .buffer_unordered(16) // Max 16 channels.
        .for_each(|_| async {})
        .await;

    Ok(())
}
