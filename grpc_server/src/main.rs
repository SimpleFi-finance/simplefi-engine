use log::info;
use tonic::transport::Server;

use grpc_server::{service::AbiDiscoveryServiceImpl, abi_discovery_proto::abi_discovery_service_server::AbiDiscoveryServiceServer};
use shared_utils::logger::init_logging;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    // TODO: Get GRPC port from settings
    let addr = "0.0.0.0:50051".parse()?;

    info!("Starting server on: {:?}", addr);

    let abi_discovery_service = AbiDiscoveryServiceImpl::default();

    let server = Server::builder()
        .add_service(AbiDiscoveryServiceServer::new(abi_discovery_service))
        .serve(addr);

    server.await.expect("Failed to start server");

    Ok(())
}
