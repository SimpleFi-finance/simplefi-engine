use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::abi_discovery_proto::abi_discovery_service_client::AbiDiscoveryServiceClient;
use crate::abi_discovery_proto::{
    DiscoverAddressesRequest, SuccessResponse, ProviderRequest, Provider, GetByChainRequest, ListProvidersResponse, GetProviderResponse, ContractAddressRequest, ContractInfoResponse, ContractsInfoResponse, ContractsAddressRequest,
};

pub struct AbiDiscoveryClient {
    client: AbiDiscoveryServiceClient<Channel>,
}

#[derive(Debug)]
pub enum ABIDiscoveryClientError {
    GRPCError(Status),
    NotFound,
    NotContractsFound,
}

impl From<Status> for ABIDiscoveryClientError {
    fn from(status: Status) -> Self {
        if status.code() == tonic::Code::NotFound {
            ABIDiscoveryClientError::NotFound
        } else {
            ABIDiscoveryClientError::GRPCError(status)
        }
    }
}

impl AbiDiscoveryClient {
    pub async fn new(address: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AbiDiscoveryServiceClient::connect(address).await?;

        Ok(Self { client })
    }

    pub async fn discover_addresses(&mut self, chain: String, addresses: Vec<String>) -> Result<SuccessResponse, ABIDiscoveryClientError> {
        let request = Request::new(DiscoverAddressesRequest {
            chain,
            addresses
        });

        let response = self.client.discover_addresses_handler(request).await?;

        Ok(response.into_inner())
    }

    pub async fn add_provider_handler(
        &mut self,
        provider: Provider
    ) -> Result<SuccessResponse, ABIDiscoveryClientError> {
        let request = Request::new(ProviderRequest {
            provider: Some(provider)
        });

        let response = self.client.add_provider_handler(request).await?;

        Ok(response.into_inner())
    }

    pub async fn list_providers_handler(
        &mut self,
        chain: String,
    ) -> Result<ListProvidersResponse, ABIDiscoveryClientError> {
        let request = Request::new(GetByChainRequest {
            chain
        });

        let response = self.client.list_providers_handler(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_provider_handler(
        &mut self,
        chain: String,
    ) -> Result<GetProviderResponse, ABIDiscoveryClientError> {
        let request = Request::new(GetByChainRequest {
            chain
        });

        let response = self.client.get_provider_handler(request).await?;

        Ok(response.into_inner())
    }

    pub async fn is_contract_tracked_handler(
        &mut self,
        chain: String,
        address: String,
    ) -> Result<SuccessResponse, ABIDiscoveryClientError> {
        let request = Request::new(ContractAddressRequest {
            chain,
            address
        });

        let response = self.client.is_contract_tracked_handler(request).await?;

        Ok(response.into_inner())
    }

    pub async fn add_contract_tracked_handler(
        &mut self,
        chain: String,
        address: String,
    ) -> Result<SuccessResponse, ABIDiscoveryClientError> {
        let request = Request::new(ContractAddressRequest {
            chain,
            address
        });

        let response = self.client.add_contract_tracked_handler(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_contract_info_handler(
        &mut self,
        chain: String,
        address: String,
    ) -> Result<ContractInfoResponse, ABIDiscoveryClientError> {
        let request = Request::new(ContractAddressRequest {
            chain,
            address
        });

        let response = self.client.get_contract_info_handler(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_contracts_info_handler(
        &mut self,
        chain: String,
        addresses: Vec<String>,
    ) -> Result<ContractsInfoResponse, ABIDiscoveryClientError> {
        print!("{:?} {:?}", &chain, &addresses);

        let request: Request<ContractsAddressRequest> = Request::new(ContractsAddressRequest {
            chain,
            addresses
        });

        let response = self.client.get_contracts_info_handler(request).await?;

        Ok(response.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_addresses() {

        let mut client: AbiDiscoveryClient  = AbiDiscoveryClient::new("http://localhost:50051".to_string()).await.unwrap();

        let request = client.get_contracts_info_handler("ethereum".to_string(), vec!["0x6b175474e89094c44da98b954eedeac495271d0f".to_string()]).await;

        let response = request.unwrap();

        assert!(response.success, "true");
        assert_eq!(response.contracts_info.len(), 1);
    }

    #[tokio::test]
    async fn test_discover_addresses_bad_address() {

        let mut client: AbiDiscoveryClient  = AbiDiscoveryClient::new("http://localhost:50051".to_string()).await.unwrap();

        let request = client.get_contracts_info_handler("ethereum".to_string(), vec!["bad_address".to_string()]).await;

        assert!(request.is_err());
    }
}
