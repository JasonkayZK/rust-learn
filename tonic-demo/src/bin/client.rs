use tonic::transport::Endpoint;
use tonic::Request;

use hello::hello_client::HelloClient;
use hello::HelloRequest;
use tonic_demo::goodbye::goodbye_client::GoodbyeClient;
use tonic_demo::goodbye::GoodbyeRequest;
use tonic_demo::hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = Endpoint::from_static("https://127.0.0.1:50051");

    let mut hello_cli = HelloClient::connect(addr.clone()).await?;
    let request = Request::new(HelloRequest {
        name: "tonic".to_string(),
    });
    let response = hello_cli.hello(request).await?;
    println!("hello response: {:?}", response.into_inner());

    let mut goodbye_cli = GoodbyeClient::connect(addr).await?;
    let request = Request::new(GoodbyeRequest {
        name: "tonic".to_string(),
    });
    let response = goodbye_cli.goodbye(request).await?;
    println!("goodbye response: {:?}", response.into_inner());

    Ok(())
}
