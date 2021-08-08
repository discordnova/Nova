mod bucket;
mod redis_client;

use std::net::SocketAddr;
pub mod nova_ratelimit_v1 {
    tonic::include_proto!("nova.ratelimit.v1");
}

use nova_ratelimit_v1::{
    ratelimit_response, ratelimit_service_server, CreateBucketData, RatelimitRequest,
    RatelimitResponse,
};

#[derive(Debug, Default)]
pub struct RatelimiterService {}

#[tonic::async_trait]
impl ratelimit_service_server::RatelimitService for RatelimiterService {
    async fn create_bucket(
        &self,
        request: tonic::Request<nova_ratelimit_v1::CreateBucketData>,
    ) -> Result<tonic::Response<nova_ratelimit_v1::CreateBucketData>, tonic::Status> {
        // todo(n1c00o): Make create_bucket endpoint implementation

        Ok(tonic::Response::new(CreateBucketData {
            limit: 0,
            remaining: 0,
            request: Some(RatelimitRequest {
                identifiers: Vec::<String>::new(),
                route_name: String::from("todo"),
            }),
            reset: 0,
        }))
    }

    async fn get_ratelimit_status(
        &self,
        request: tonic::Request<nova_ratelimit_v1::RatelimitRequest>,
    ) -> Result<tonic::Response<nova_ratelimit_v1::RatelimitResponse>, tonic::Status> {
        // todo(n1c00o): Make get_ratelimit_status endpoint implementation

        Ok(tonic::Response::new(RatelimitResponse {
            status: ratelimit_response::Status::Ok as i32,
            update_asked: false,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::0]:50051".parse()?; // Use ::0 and not ::1 because it does not work in Docker

    println!("Ratelimiter service listening on {}", addr);
    Ok(())
}
