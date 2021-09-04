// Some implementation of the gRPC service using the shared library.

pub mod ratelimit_pb {
    tonic::include_proto!("nova.ratelimit.v1");
}

use ratelimit_pb::ratelimit_service_server::{RatelimitService, RatelimitServiceServer};
use ratelimit_pb::{CreateBucketData, RatelimitResponse, RatelimitRequest};
use tonic::{Request, Status, Response};
use tonic::transport::Server;
use std::error::Error;

#[derive(Default)]
pub struct MyRatelimitService {}

#[tonic::async_trait]
impl RatelimitService for MyRatelimitService {
    async fn get_ratelimit_status(
        &self,
        _request: Request<RatelimitRequest>
    ) -> Result<Response<RatelimitResponse>, Status> {
        return Err(Status::not_found("Not implmented"))
    }
    async fn create_bucket(
        &self,
        _request: Request<CreateBucketData>
    ) ->Result<Response<CreateBucketData>, Status> {
        return Err(tonic::Status::not_found("Not implmented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service = MyRatelimitService::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(RatelimitServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}