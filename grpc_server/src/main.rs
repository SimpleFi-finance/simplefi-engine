
use tonic::{transport::Server, Request, Response, Status};

use abi_discovery::helpers::{get_tracked_abis, add_factory_addresses};
use grpc_server::abi_discovery_proto::abi_discovery_service_server::{ AbiDiscoveryService, AbiDiscoveryServiceServer };
use grpc_server::abi_discovery_proto::{
    TrackedAddressesRequest,
    TrackedAddressesResponse,
    AddressAbi,
    GetAddressesAbiRequest,
    GetAddressesAbiResponse,
    AddFactoryAddressesRequest,
    AddFactoryAddressesResponse,
};
use shared_types::redis::abi::ContractWithAbiRedis;


#[derive(Default)]
pub struct AbiDiscoveryServiceImpl {}

#[tonic::async_trait]
impl AbiDiscoveryService for AbiDiscoveryServiceImpl {
    async fn check_tracked_addresses(
        &self,
        request: Request<TrackedAddressesRequest>,
    ) -> Result<Response<TrackedAddressesResponse>, Status> {
        let addresses = request.into_inner().addresses;

        let response = TrackedAddressesResponse { addresses };
        Ok(Response::new(response))
    }

    async fn get_addresses_abi(
        &self,
        request: Request<GetAddressesAbiRequest>,
    ) -> Result<Response<GetAddressesAbiResponse>, Status> {
        let addresses = request.into_inner().addresses;

        let tracked_abis = get_tracked_abis(addresses).await.expect("Failed to get tracked abis");

        let mut contract_abis: Vec<AddressAbi> = Vec::new();

        // loop through keys and values
        for (key, value) in tracked_abis.iter() {
            println!("key: {}, value: {}", key, value);

            let deserialized_value: ContractWithAbiRedis = serde_json::from_str(value.as_str()).expect("Failed to deserialize value");

            let single_response = AddressAbi {
                timestamp: deserialized_value.timestamp,
                address: key.to_string(),
                abi: deserialized_value.abi,
            };

            contract_abis.push(single_response);
        };

        let response = GetAddressesAbiResponse {
            addresses_abi: contract_abis, // vec![single_response]
        };

        Ok(Response::new(response))
    }

    async fn add_factory_addresses(
        &self,
        request: Request<AddFactoryAddressesRequest>,
    ) -> Result<Response<AddFactoryAddressesResponse>, Status> {
        let factory_address = request.get_ref().factory_address.clone();
        let addresses = request.get_ref().addresses.clone();

        println!("factory: {:?}", factory_address);
        println!("addresses {:?}", addresses);

        let response = add_factory_addresses(factory_address, addresses).await.expect("Failed to add factory addresses");

        let response = AddFactoryAddressesResponse {
            success: response
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create tonic server
    let addr = "[::1]:50051".parse()?;

    let abi_discovery_service = AbiDiscoveryServiceImpl::default();

    let server = Server::builder()
        .add_service(AbiDiscoveryServiceServer::new(abi_discovery_service))
        .serve(addr);

    server.await.expect("Failed to start server");

    Ok(())
}
