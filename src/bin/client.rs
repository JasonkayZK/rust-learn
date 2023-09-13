use std::env;

use rust_learn::storage_proto::storage_client::StorageClient;
use rust_learn::storage_proto::{AddRequest, ListRequest};
use rust_learn::utils::get_port;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_port = get_port(env::args().collect());
    let addr = format!("http://0.0.0.0:{}", server_port);

    let mut cli = StorageClient::connect(addr).await.unwrap();
    cli.add(AddRequest {
        key: boost_rs::rand::string::get_random_alphanumeric_string(3),
    })
    .await
    .unwrap();
    println!(
        "list: {:#?}",
        cli.list(ListRequest {}).await.unwrap().into_inner().data
    );

    // cli.register(RegisterRequest { connect_addr: "192.168.31.22:8888".to_string() }).await.unwrap();

    Ok(())
}
