use log::{ info, debug, warn };
use tonic::{transport::Server, Request, Response, Status};

use abi_discovery::helpers::{
    get_tracked_abis,
    get_tracked_abis_json,
    add_factory_addresses,
    get_signatures_event,
    get_four_byte_signatures_event
};
use grpc_server::abi_discovery_proto::{
    TrackedAddressesRequest,
    TrackedAddressesResponse,
    AddressAbi,
    GetAddressesAbiRequest,
    GetAddressesAbiResponse,
    AddFactoryAddressesRequest,
    AddFactoryAddressesResponse,
    AddressAbiJson,
    GetAddressesAbiJsonResponse,
    GetAbiEventsRequest,
    GetAbiEventsResponse,
    AbiEvent,
    GetFourByteEventsRequest,
    GetFourByteEventsResponse,
    FourByteEvent,
    abi_discovery_service_server::{ AbiDiscoveryService, AbiDiscoveryServiceServer }
};
use shared_types::redis::abi::{ContractWithAbiRedis, ContractWithAbiJSONRedis};
use shared_utils::logger::init_logging;

#[derive(Default)]
pub struct AbiDiscoveryServiceImpl {}

#[tonic::async_trait]
impl AbiDiscoveryService for AbiDiscoveryServiceImpl {
    async fn check_tracked_addresses(
        &self,
        request: Request<TrackedAddressesRequest>,
    ) -> Result<Response<TrackedAddressesResponse>, Status> {
        let addresses = request.into_inner().addresses;

        info!("check_tracked_addresses called: {:?}", addresses);

        let response = TrackedAddressesResponse { addresses };
        Ok(Response::new(response))
    }

    async fn get_addresses_abi(
        &self,
        request: Request<GetAddressesAbiRequest>,
    ) -> Result<Response<GetAddressesAbiResponse>, Status> {
        let addresses = request.into_inner().addresses;

        info!("get_addresses_abi called: {:?}", addresses);

        let tracked_abis = get_tracked_abis(addresses).await.expect("Failed to get tracked abis");

        let mut contract_abis: Vec<AddressAbi> = Vec::new();

        // loop through keys and values
        for (key, value) in tracked_abis.iter() {
            debug!("key: {}, value: {}", key, value);

            let deserialized_value: ContractWithAbiRedis = serde_json::from_str(value.as_str()).expect("Failed to deserialize value");

            let single_response = AddressAbi {
                timestamp: deserialized_value.timestamp,
                address: key.to_string(),
                abi: deserialized_value.abi,
            };

            contract_abis.push(single_response);
        };

        let response = GetAddressesAbiResponse {
            addresses_abi: contract_abis,
        };

        Ok(Response::new(response))
    }

    async fn add_factory_addresses(
        &self,
        request: Request<AddFactoryAddressesRequest>,
    ) -> Result<Response<AddFactoryAddressesResponse>, Status> {
        let factory_address = request.get_ref().factory_address.clone();
        let addresses = request.get_ref().addresses.clone();

        info!("add_factory_addresses called: {:?}, {:?}", factory_address, addresses);

        let response = add_factory_addresses(factory_address, addresses).await.expect("Failed to add factory addresses");

        let response = AddFactoryAddressesResponse {
            success: response
        };

        Ok(Response::new(response))
    }

    async fn get_addresses_abi_json(
        &self,
        request: Request<GetAddressesAbiRequest>,
    ) -> Result<Response<GetAddressesAbiJsonResponse>, Status> {
        let addresses = request.into_inner().addresses;

        info!("get_addresses_abi_json called: {:?}", addresses);

        let tracked_abis = get_tracked_abis_json(addresses).await.expect("Failed to get tracked abis");

        info!("tracked_abis: {:?}", tracked_abis);

        let mut contract_abis: Vec<AddressAbiJson> = Vec::new();

        // loop through keys and values
        for (key, value) in tracked_abis.iter() {
            debug!("key: {}, value: {}", key, value);

            let deserialized_value: ContractWithAbiJSONRedis = serde_json::from_str(value.as_str()).expect("Failed to deserialize value");

            let single_response = AddressAbiJson {
                timestamp: deserialized_value.timestamp,
                address: key.to_string(),
                abi: deserialized_value.abi,
            };

            contract_abis.push(single_response);
        };

        let response = GetAddressesAbiJsonResponse {
            addresses_abi: contract_abis,
        };

        Ok(Response::new(response))
    }

    async fn get_signatures_event(
        &self,
        request: Request<GetAbiEventsRequest>,
    ) -> Result<Response<GetAbiEventsResponse>, Status> {
        let signatures = request.into_inner().signatures;

        warn!("get_signatures_event called: {:?}", signatures);

        let signatures_event = get_signatures_event(&signatures).await.expect("Failed to get signature events");

        debug!("signature events: {:?}", signatures_event);

        let mut signatures: Vec<AbiEvent> = Vec::new();

        for event in signatures_event.iter() {
            debug!("event: {:?}", event);

            let single_response = AbiEvent {
                timestamp: event.timestamp,
                name: event.name.clone(),
                signature: event.signature.clone(),
                event: event.event.clone(),
            };

            signatures.push(single_response);
        };

        let response = GetAbiEventsResponse {
            signatures_event: signatures,
        };

        Ok(Response::new(response))
    }

    async fn get_four_byte_signatures_event(
        &self,
        request: Request<GetFourByteEventsRequest>,
    ) -> Result<Response<GetFourByteEventsResponse>, Status> {
        let signatures = request.into_inner().signatures;

        warn!("get_signatures_event called: {:?}", signatures);

        let signatures_event = get_four_byte_signatures_event(&signatures).await.expect("Failed to get signature events");

        debug!("signature events: {:?}", signatures_event);

        let mut signatures: Vec<FourByteEvent> = Vec::new();

        for event in signatures_event.iter() {
            debug!("event: {:?}", event);

            let single_response = FourByteEvent {
                id: event.id,
                timestamp: event.timestamp,
                text_signature: event.text_signature.clone(),
                hex_signature: event.hex_signature.clone(),
            };

            signatures.push(single_response);
        };

        let response = GetFourByteEventsResponse {
            signatures_event: signatures,
        };

        Ok(Response::new(response))
    }
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    // create tonic server
    let addr = "[::1]:50051".parse()?;

    info!("Starting server on: {:?}", addr);

    let abi_discovery_service = AbiDiscoveryServiceImpl::default();

    let server = Server::builder()
        .add_service(AbiDiscoveryServiceServer::new(abi_discovery_service))
        .serve(addr);

    server.await.expect("Failed to start server");

    info!("Server started successfully");

    Ok(())
}
