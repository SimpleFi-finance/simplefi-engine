
use tonic::{transport::Server, Request, Response, Status};

pub mod abi_discovery_proto {
    tonic::include_proto!("abi_discovery_proto");
}

use abi_discovery_proto::abi_discovery_service_server::{AbiDiscoveryService, AbiDiscoveryServiceServer};
use abi_discovery_proto::{TrackedAddressesRequest, TrackedAddressesResponse};

#[derive(Default)]
pub struct MyAbiDiscoveryService {}

#[tonic::async_trait]
impl AbiDiscoveryService for MyAbiDiscoveryService {
    async fn check_tracked_addresses(
        &self,
        request: Request<TrackedAddressesRequest>,
    ) -> Result<Response<TrackedAddressesResponse>, Status> {
        let addresses = request.into_inner().addresses;

        let response = TrackedAddressesResponse { addresses };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "127.0.0.1:50051".parse()?;
    let addr = "[::1]:50051".parse()?;
    let abi_discovery_service = MyAbiDiscoveryService::default();

    Server::builder()
        .add_service(AbiDiscoveryServiceServer::new(abi_discovery_service))
        .serve(addr)
        .await?;

    Ok(())
}

