use log::{debug, info};
use simplefi_engine_settings::load_settings;
use std::collections::HashMap;
use tonic::{Request, Response, Status};

use crate::abi_discovery_proto::{
    abi_discovery_service_server::AbiDiscoveryService, ContractAddressRequest,
    ContractInfoResponse, DiscoverAddressesRequest, GetByChainRequest, GetProviderResponse,
    ListProvidersResponse, ProviderRequest, SuccessResponse, ContractsAddressRequest, ContractsInfoResponse,
};
use abi_discovery::{helpers::{contracts, providers, providers::Provider}, mongo::types::{ContractAbiCollection, AbiCollection}};

use mongo_types::{ Mongo, MongoConfig };
use simplefi_redis::connect;

#[derive(Default)]
pub struct AbiDiscoveryServiceImpl {}

#[tonic::async_trait]
impl AbiDiscoveryService for AbiDiscoveryServiceImpl {
    async fn discover_addresses_handler(
        &self,
        request: Request<DiscoverAddressesRequest>,
    ) -> Result<Response<SuccessResponse>, Status> {
        let req = request.into_inner();

        let chain = req.chain;
        let addresses = req.addresses;

        debug!("discover_addresses called: {:?} - {:?}", chain, addresses);

        if chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        if addresses.len() == 0 {
            return Err(Status::invalid_argument("No addresses specified"));
        }

        let mysettings = load_settings().expect("Failed to load settings");
        let redis_uri = mysettings.redis_uri.to_string();

        let mut con = connect(&redis_uri.as_str())
            .await
            .expect("Failed to connect to redis");

        let response = contracts::discover(&mut con, &chain, addresses).await;

        let response = SuccessResponse {
            success: response.is_ok(),
        };

        Ok(Response::new(response))
    }

    async fn add_provider_handler(
        &self,
        request: Request<ProviderRequest>,
    ) -> Result<Response<SuccessResponse>, Status> {
        let req = request.into_inner();

        let provider = match req.provider {
            Some(provider) => provider,
            None => return Err(Status::invalid_argument("No provider specified")),
        };

        if provider.name.len() == 0 {
            return Err(Status::invalid_argument("No name specified"));
        }
        if provider.name.len() > 8 {
            return Err(Status::invalid_argument(
                "Name too long. Maximum 8 characters",
            ));
        } else if provider.name.contains(" ") {
            return Err(Status::invalid_argument("Name contains empty spaces"));
        }

        if provider.chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        if provider.provider_type.len() == 0 {
            return Err(Status::invalid_argument("No provider_type specified"));
        }

        let rate_limits: HashMap<String, u32> = provider
            .rate_limits
            .into_iter()
            .filter_map(|(k, v)| match v.parse::<u32>() {
                Ok(n) => Some((k, n)),
                Err(e) => {
                    eprintln!("Failed to convert string to u32: {}", e);
                    None
                }
            })
            .collect();

        let mysettings = load_settings().expect("Failed to load settings");
        let redis_uri = mysettings.redis_uri.to_string();

        let mut con = connect(&redis_uri.as_str())
            .await
            .expect("Failed to connect to redis");

        let response = providers::add(
            &mut con,
            Provider {
                name: provider.name.to_lowercase(),
                chain: provider.chain.to_lowercase(),
                provider_type: provider.provider_type.to_lowercase(),
                api_key: provider.api_key,
                rate_limits,
            },
        )
        .await;

        info!("add_provider added");

        Ok(Response::new(SuccessResponse {
            success: response.is_ok(),
        }))
    }

    async fn list_providers_handler(
        &self,
        request: Request<GetByChainRequest>,
    ) -> Result<Response<ListProvidersResponse>, Status> {
        let req = request.into_inner();
        let chain = req.chain;

        if chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        let mysettings = load_settings().expect("Failed to load settings");
        let redis_uri = mysettings.redis_uri.to_string();

        let mut con = connect(&redis_uri.as_str())
            .await
            .expect("Failed to connect to redis");

        let response = providers::list(&mut con, chain).await;

        if response.is_err() {
            return Err(Status::invalid_argument("Failed to read provider list"));
        }

        let success = response.is_ok();

        let mut providers: Vec<crate::abi_discovery_proto::Provider> = Vec::new();

        for provider in response.unwrap().iter() {
            providers.push(crate::abi_discovery_proto::Provider {
                chain: provider.chain.to_string(),
                name: provider.name.to_string(),
                provider_type: provider.provider_type.to_string(),
                api_key: provider.api_key.to_string(),
                rate_limits: provider
                    .rate_limits
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            });
        }

        Ok(Response::new(ListProvidersResponse { success, providers }))
    }

    async fn get_provider_handler(
        &self,
        request: Request<GetByChainRequest>,
    ) -> Result<Response<GetProviderResponse>, Status> {
        let req = request.into_inner();

        let chain = req.chain;

        if chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        let mysettings = load_settings().expect("Failed to load settings");
        let redis_uri = mysettings.redis_uri.to_string();

        let mut con = simplefi_redis::connect(&redis_uri.as_str())
            .await
            .expect("Failed to connect to redis");

        let response = providers::get_available_provider(&mut con, chain.as_str()).await;

        if response.is_err() {
            return Err(Status::invalid_argument(
                response.err().unwrap().to_string(),
            ));
        }

        let success = response.is_ok();
        let provider = response.unwrap();

        info!("get_provider called: {:?}", provider);

        let provider_output = crate::abi_discovery_proto::Provider {
            chain: provider.chain.to_string(),
            name: provider.name.to_string(),
            provider_type: provider.provider_type.to_string(),
            api_key: provider.api_key.to_string(),
            rate_limits: provider
                .rate_limits
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        };

        Ok(Response::new(GetProviderResponse {
            success,
            provider: Some(provider_output),
        }))
    }

    async fn is_contract_tracked_handler(
        &self,
        request: Request<ContractAddressRequest>,
    ) -> Result<Response<SuccessResponse>, Status> {
        let req = request.into_inner();
        let chain = req.chain;
        let address = req.address;

        if chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        if address.len() == 0 {
            return Err(Status::invalid_argument("No address specified"));
        }

        let mysettings = load_settings().expect("Failed to load settings");
        let redis_uri = mysettings.redis_uri.to_string();

        let mut con = simplefi_redis::connect(&redis_uri.as_str())
            .await
            .expect("Failed to connect to redis");

        let response = contracts::is_tracked(&mut con, chain.as_str(), address.as_str()).await;

        if response.is_err() {
            return Err(Status::invalid_argument(
                response.err().unwrap().to_string(),
            ));
        }

        let success = response.unwrap();

        debug!("is_contract_tracked called: {:?}", success);

        Ok(Response::new(SuccessResponse { success }))
    }

    async fn add_contract_tracked_handler(
        &self,
        request: Request<ContractAddressRequest>,
    ) -> Result<Response<SuccessResponse>, Status> {
        let req = request.into_inner();
        let chain = req.chain;
        let address = req.address;

        if chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        if address.len() == 0 {
            return Err(Status::invalid_argument("No address specified"));
        }

        let mysettings = load_settings().expect("Failed to load settings");
        let redis_uri = mysettings.redis_uri.to_string();

        let mut con = simplefi_redis::connect(&redis_uri.as_str())
            .await
            .expect("Failed to connect to redis");

        let response = contracts::add_tracked(&mut con, chain.as_str(), address.as_str()).await;

        if response.is_err() {
            return Err(Status::invalid_argument(
                response.err().unwrap().to_string(),
            ));
        }

        let success = response.unwrap();

        debug!("add_contract_tracked called: {:?}", success);

        Ok(Response::new(SuccessResponse { success }))
    }

    async fn get_contract_info_handler(
        &self,
        request: Request<ContractAddressRequest>,
    ) -> Result<Response<ContractInfoResponse>, Status> {
        let req = request.into_inner();
        let chain = req.chain;
        let address = req.address.clone();

        if chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        if address.len() == 0 {
            return Err(Status::invalid_argument("No address specified"));
        }

        let mysettings = load_settings().expect("Failed to load settings");
        let redis_uri = mysettings.redis_uri.to_string();

        let config = MongoConfig {
            uri: mysettings.mongodb_uri.to_string(),
            database: mysettings.mongodb_database_name.to_string(),
        };

        let mongo = Mongo::new(&config).await.unwrap();

        let mut redis_connection = connect(&redis_uri.as_str())
            .await
            .expect("Failed to connect to redis");

        let contracts_abi_collection_name =
            format!("{}_{}", &chain, &mysettings.contract_abi_collection_name);

        let abis_collection = mongo.database.collection::<AbiCollection>(
            format!("{}_{}", &chain, &mysettings.abi_collection_name).as_str(),
        );
        let contracts_collection = mongo
            .database
            .collection::<ContractAbiCollection>(contracts_abi_collection_name.as_str());

        let response = contracts::get_full_contracts_info(
            &mut redis_connection,
            &contracts_collection,
            &abis_collection,
            &chain,
            [address].to_vec(),
        )
        .await;

        if response.is_err() {
            return Err(Status::invalid_argument(
                response.err().unwrap().to_string(),
            ));
        }

        let response = response.unwrap();

        let contract_info = if response.len() > 0 {
            let contract = response.get(0).unwrap();

            let mut contract_implementations = Vec::new();

            for implementation in contract.implementations.iter() {
                let abi_implementation = if implementation.abi.is_some() {
                    let abi = implementation.abi.as_ref().unwrap();

                    Some(crate::abi_discovery_proto::Abi {
                        abi: abi.abi.to_string(),
                        abi_hash: abi.abi_hash.to_string(),
                        is_proxy: abi.is_proxy,
                        standard: abi.standard.as_u32() as i32,
                    })
                } else {
                    None
                };

                let creation_block = if implementation.creation_block.is_some() {
                    implementation.creation_block.as_ref().unwrap().to_string()
                } else {
                    "".to_string()
                };

                contract_implementations.push(crate::abi_discovery_proto::ContractImplementation {
                    name: implementation.name.to_string(),
                    address: implementation.address.to_string(),
                    abi: abi_implementation,
                    creation_block: creation_block,
                    verified: implementation.verified,
                });
            }

            let contract_abi = if contract.abi.is_some() {
                let abi = contract.abi.as_ref().unwrap();

                Some(crate::abi_discovery_proto::Abi {
                    abi: abi.abi.to_string(),
                    abi_hash: abi.abi_hash.to_string(),
                    is_proxy: abi.is_proxy,
                    standard: abi.standard.as_u32() as i32,
                })
            } else {
                None
            };

            let creation_block = if contract.creation_block.is_some() {
                contract.creation_block.as_ref().unwrap().to_string()
            } else {
                "".to_string()
            };

            Some(crate::abi_discovery_proto::ContractInfo {
                chain: chain.to_string(),
                name: contract.name.to_string(),
                address: contract.address.to_string(),
                abi: contract_abi,
                creation_block: creation_block,
                is_proxy: contract.is_proxy,
                verified: contract.verified,
                implementations: contract_implementations,
            })
        } else {
            None
        };

        Ok(Response::new(ContractInfoResponse {
            success: response.len() > 0,
            contract_info,
        }))
    }

    async fn get_contracts_info_handler(
        &self,
        request: Request<ContractsAddressRequest>,
    ) -> Result<Response<ContractsInfoResponse>, Status> {
        let req = request.into_inner();

        let chain = req.chain;
        let addresses = req.addresses.clone();

        if chain.len() == 0 {
            return Err(Status::invalid_argument("No chain specified"));
        }

        if addresses.len() == 0 {
            return Err(Status::invalid_argument("No addresses specified"));
        }

        let mysettings = load_settings().expect("Failed to load settings");

        let config = MongoConfig {
            uri: mysettings.mongodb_uri.to_string(),
            database: mysettings.mongodb_database_name.to_string(),
        };

        let mongo = Mongo::new(&config).await.unwrap();

        let mut redis_connection = connect(&mysettings.redis_uri)
            .await
            .expect("Failed to connect to redis");

        let contracts_abi_collection_name =
            format!("{}_{}", &chain, &mysettings.contract_abi_collection_name);

        let abis_collection = mongo.database.collection::<AbiCollection>(
            format!("{}_{}", &chain, &mysettings.abi_collection_name).as_str(),
        );
        let contracts_collection = mongo
            .database
            .collection::<ContractAbiCollection>(contracts_abi_collection_name.as_str());

        let response = contracts::get_full_contracts_info(
            &mut redis_connection,
            &contracts_collection,
            &abis_collection,
            &chain,
            addresses,
        ).await;

        if response.is_err() {
            return Err(Status::invalid_argument(
                response.err().unwrap().to_string(),
            ));
        }

        let response = response.unwrap();

        if response.len() == 0 {
            return Err(Status::invalid_argument(
                "No contracts found for the specified addresses",
            ));
        }

        let mut contracts_info = Vec::new();

        for contract in response {
            let mut contract_implementations = Vec::new();

            for implementation in contract.implementations.iter() {
                let abi_implementation = if implementation.abi.is_some() {
                    let abi = implementation.abi.as_ref().unwrap();

                    Some(crate::abi_discovery_proto::Abi {
                        abi: abi.abi.to_string(),
                        abi_hash: abi.abi_hash.to_string(),
                        is_proxy: abi.is_proxy,
                        standard: abi.standard.as_u32() as i32,
                    })
                } else {
                    None
                };

                let creation_block = if implementation.creation_block.is_some() {
                    implementation.creation_block.as_ref().unwrap().to_string()
                } else {
                    "".to_string()
                };

                contract_implementations.push(crate::abi_discovery_proto::ContractImplementation {
                    name: implementation.name.to_string(),
                    address: implementation.address.to_string(),
                    abi: abi_implementation,
                    creation_block: creation_block,
                    verified: implementation.verified,
                });
            }

            let contract_abi = if contract.abi.is_some() {
                let abi = contract.abi.as_ref().unwrap();

                Some(crate::abi_discovery_proto::Abi {
                    abi: abi.abi.to_string(),
                    abi_hash: abi.abi_hash.to_string(),
                    is_proxy: abi.is_proxy,
                    standard: abi.standard.as_u32() as i32,
                })
            } else {
                None
            };

            let creation_block = if contract.creation_block.is_some() {
                contract.creation_block.as_ref().unwrap().to_string()
            } else {
                "".to_string()
            };

            contracts_info.push(crate::abi_discovery_proto::ContractInfo {
                chain: chain.to_string(),
                name: contract.name.to_string(),
                address: contract.address.to_string(),
                abi: contract_abi,
                creation_block: creation_block,
                is_proxy: contract.is_proxy,
                verified: contract.verified,
                implementations: contract_implementations,
            });
        };

        Ok(Response::new(ContractsInfoResponse {
            success: true,
            contracts_info,
        }))
    }
}
