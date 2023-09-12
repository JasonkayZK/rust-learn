use rust_learn::api::StorageClient;
use rust_learn::utils::get_port;
use std::env;
use tarpc::tokio_serde::formats::Json;
use tarpc::{client, context};

#[tokio::main]
async fn main() {
    let server_port = get_port(env::args().collect());

    let to_storage_server =
        tarpc::serde_transport::tcp::connect(format!("0.0.0.0:{}", server_port), Json::default)
            .await
            .unwrap();
    let storage_client = StorageClient::new(client::Config::default(), to_storage_server).spawn();

    let ctx = context::current();
    storage_client
        .add(
            ctx,
            boost_rs::rand::string::get_random_alphanumeric_string(3),
        )
        .await
        .unwrap();
    println!("list: {:#?}", storage_client.list(ctx).await.unwrap())
}
