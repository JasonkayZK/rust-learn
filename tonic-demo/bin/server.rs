use tonic::transport::Server;
use tonic::{Request, Response, Status};

use hello::hello_server::{Hello, HelloServer};
use hello::{HelloRequest, HelloResponse};
use tonic_demo::hello;

#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl Hello for MyServer {
    async fn hello_world(
        &self,
        res: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let response = HelloResponse {
            message: format!("Hello, {}", res.into_inner().name),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    println!("server starting at: {}", addr);

    let hello_server = MyServer::default();
    Server::builder()
        .add_service(HelloServer::new(hello_server))
        .serve(addr)
        .await?;

    Ok(())
}
