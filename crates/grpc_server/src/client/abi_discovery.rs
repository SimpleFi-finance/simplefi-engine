use tonic::transport::Channel;
use tonic::{Request, Response};

use crate::abi_discovery_proto::abi_discovery_service_client::AbiDiscoveryServiceClient;
use crate::abi_discovery_proto::{
    DiscoverAddressesRequest, SuccessResponse, ProviderRequest, Provider, GetByChainRequest, ListProvidersResponse, GetProviderResponse, ContractAddressRequest, ContractInfoResponse, ContractsInfoResponse, ContractsAddressRequest,
};

pub struct AbiDiscoveryClient {
    client: AbiDiscoveryServiceClient<Channel>,
}

impl AbiDiscoveryClient {
    pub async fn new(address: String) -> Self {
        let client = AbiDiscoveryServiceClient::connect(address).await.unwrap();

        Self { client }
    }

    pub async fn discover_addresses(&mut self, chain: String, addresses: Vec<String>) -> Response<SuccessResponse> {
        let request = Request::new(DiscoverAddressesRequest {
            chain,
            addresses
        });

        let response = self.client.discover_addresses_handler(request).await.unwrap();

        response
    }

    pub async fn add_provider_handler(
        &mut self,
        provider: Provider
    ) -> Response<SuccessResponse> {
        let request = Request::new(ProviderRequest {
            provider: Some(provider)
        });

        let response = self.client.add_provider_handler(request).await.unwrap();

        response
    }

    pub async fn list_providers_handler(
        &mut self,
        chain: String,
    ) -> Response<ListProvidersResponse> {
        let request = Request::new(GetByChainRequest {
            chain
        });

        let response = self.client.list_providers_handler(request).await.unwrap();

        response
    }

    pub async fn get_provider_handler(
        &mut self,
        chain: String,
    ) -> Response<GetProviderResponse> {
        let request = Request::new(GetByChainRequest {
            chain
        });

        let response = self.client.get_provider_handler(request).await.unwrap();

        response
    }

    pub async fn is_contract_tracked_handler(
        &mut self,
        chain: String,
        address: String,
    ) -> Response<SuccessResponse> {
        let request = Request::new(ContractAddressRequest {
            chain,
            address
        });

        let response = self.client.is_contract_tracked_handler(request).await.unwrap();

        response
    }

    pub async fn add_contract_tracked_handler(
        &mut self,
        chain: String,
        address: String,
    ) -> Response<SuccessResponse> {
        let request = Request::new(ContractAddressRequest {
            chain,
            address
        });

        let response = self.client.add_contract_tracked_handler(request).await.unwrap();

        response
    }

    pub async fn get_contract_info_handler(
        &mut self,
        chain: String,
        address: String,
    ) -> Response<ContractInfoResponse> {
        let request = Request::new(ContractAddressRequest {
            chain,
            address
        });

        let response = self.client.get_contract_info_handler(request).await.unwrap();

        response
    }

    pub async fn get_contracts_info_handler(
        &mut self,
        chain: String,
        addresses: Vec<String>,
    ) -> Response<ContractsInfoResponse> {
        let request = Request::new(ContractsAddressRequest {
            chain,
            addresses
        });

        let response = self.client.get_contracts_info_handler(request).await.unwrap();

        response
    }
}
