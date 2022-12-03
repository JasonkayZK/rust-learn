use tonic::transport::Server;
use tonic::{Request, Response, Status};

use hello::hello_server::{Hello, HelloServer};
use hello::{HelloRequest, HelloResponse};
use tonic_demo::basic::BaseResponse;
use tonic_demo::goodbye::goodbye_server::{Goodbye, GoodbyeServer};
use tonic_demo::goodbye::{GoodbyeRequest, GoodbyeResponse};
use tonic_demo::hello;

#[derive(Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let response = HelloResponse {
            data: format!("Hello, {}", req.into_inner().name),
            message: Some(BaseResponse {
                message: "Ok".to_string(),
                code: 200,
            }),
        };
        Ok(Response::new(response))
    }
}

#[derive(Default)]
pub struct GoodbyeService {}

#[tonic::async_trait]
impl Goodbye for GoodbyeService {
    async fn goodbye(
        &self,
        req: Request<GoodbyeRequest>,
    ) -> Result<Response<GoodbyeResponse>, Status> {
        let response = GoodbyeResponse {
            data: format!("Goodbye, {}", req.into_inner().name),
            message: Some(BaseResponse {
                message: "Ok".to_string(),
                code: 200,
            }),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    println!("server starting at: {}", addr);

    Server::builder()
        .add_service(HelloServer::new(HelloService::default()))
        .add_service(GoodbyeServer::new(GoodbyeService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
