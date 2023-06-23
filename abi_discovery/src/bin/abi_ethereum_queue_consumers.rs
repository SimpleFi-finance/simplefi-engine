use log::{ error, debug };
use redis::{AsyncCommands, RedisError};
use settings::load_settings;
use tokio::{time::{ sleep, Duration }, spawn};

use abi_discovery::{
    helpers::{
        abis::get_abi_standard,
        contracts::{get_contracts_queue_name, get_contract_abi},
        providers::get_available_provider,
    },
    mongo::{types::{AbiCollection, ContractAbiCollection, ImplementationContractAbiCollection}, setters::{insert_contract, insert_abi}}
};
use shared_utils::logger::init_logging;
use shared_utils::{
    redis::{connect, has_items_in_queue}
};

use shared_types::mongo::{ MongoConfig, Mongo };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let chain = "ethereum";

    let mysettings = load_settings().expect("Failed to load settings");

    let config = MongoConfig {
        uri: mysettings.mongodb_uri,
        database: mysettings.mongodb_database_name,
    };
    let mongo = Mongo::new(&config).await.unwrap();

    let redis_uri = mysettings.redis_uri.to_string();
    let redis_queue = get_contracts_queue_name(chain);
    let mut redis_connection = connect(&redis_uri.as_str())
        .await
        .expect("Failed to connect to redis");

    loop {
        let abis_collection = mongo.database.collection::<AbiCollection>(&mysettings.abi_collection_name);
        let contracts_collection = mongo.database.collection::<ContractAbiCollection>(&mysettings.contract_abi_collection_name);

        let has_items = has_items_in_queue(&mut redis_connection, &redis_queue).await;

        if has_items.is_err() {
            error!("Failed to check if there are items in queue: {:?}", has_items.unwrap_err());

            sleep(Duration::from_secs(1)).await;

            continue;
        } else if has_items.unwrap() == false {
            debug!("No contracts in queue. waiting...");

            sleep(Duration::from_secs(1)).await;

            continue;
        }

        let chain = "ethereum";

        let provider = get_available_provider(&mut redis_connection, &chain).await;

        if provider.is_err() {
            error!("Failed to get provider: {:?}", provider.err().unwrap());

            sleep(Duration::from_secs(1)).await;

            continue;
        }

        let provider = provider.unwrap();

        debug!("available provider");

        let contract: Result<String, RedisError> = redis_connection.spop(&redis_queue).await;

        if contract.is_err() {
            error!("Failed to get contract from redis queue");

            sleep(Duration::from_secs(1)).await;

            continue;
        }

        let contract = contract.unwrap();

        debug!("Contract popped: {:?}", contract);

        let contract_info = get_contract_abi(&provider, &contract).await;

        if contract_info.is_err() {
            error!("Failed to get contract info: {:?}", contract_info.unwrap_err());

            continue;
        }

        let contract_info = contract_info.unwrap();

        debug!("contract_info: {:?}", contract_info);

        if contract_info.abi.len() == 0 || contract_info.abi == "Contract source code not verified" {
            debug!("No Abi found for contract: {:?}", contract);

            spawn(async move {
                let _ = insert_contract(
                    &contracts_collection,
                    ContractAbiCollection {
                        id: None,
                        name: "Unknown".to_string(),
                        address: contract,
                        abi_id: None,
                        creation_block: None,
                        verified: false,
                        is_proxy: false,
                        implementations: vec![],
                }).await;
            });

            continue;
        }

        let abi_is_proxy = contract_info.proxy == "1";

        let abi_standard = get_abi_standard(&contract_info.abi);

        let abi_id = insert_abi(
            &abis_collection,
            &contract_info.abi.as_str(),
            &abi_is_proxy.to_owned(),
            &abi_standard.as_u32(),
        ).await;

        if abi_id.is_err() {
            error!("Failed to insert abi: {:?}", abi_id.unwrap_err());

            continue;
        }

        let abi_id = abi_id.unwrap();

        if contract_info.proxy == "0" {
            debug!("Contract is not proxy");

            spawn(async move {
                let _ = insert_contract(
                    &contracts_collection,
                    ContractAbiCollection {
                        id: None,
                        name: contract_info.contract_name,
                        address: contract,
                        abi_id: Some(abi_id),
                        creation_block: None,
                        verified: true,
                        is_proxy: false,
                        implementations: vec![],
                }).await;
            });

            continue;
        } else {
            debug!("Contract is proxy");

            let implementation_contract_info = get_contract_abi(&provider, &contract_info.implementation).await;

            if implementation_contract_info.is_err() {
                error!("Failed to get implementation abi: {:?}", implementation_contract_info.unwrap_err());

                continue;
            }

            let implementation_contract_info = implementation_contract_info.unwrap();

            if implementation_contract_info.abi.len() == 0 {
                error!("No Abi found for implementation contract: {:?}", contract_info.implementation);
            }

            let implementation_abi_standard = get_abi_standard(&implementation_contract_info.abi);

            let implementation_abi = insert_abi(
                &abis_collection,
                &implementation_contract_info.abi.as_str(),
                &false,
                &implementation_abi_standard.as_u32(),
            ).await;

            if implementation_abi.is_err() {
                error!("Failed to insert implementation abi: {:?}", implementation_abi.unwrap_err());

                continue;
            }

            let implementation_abi_id = implementation_abi.unwrap();

            let implemetation_contract = ImplementationContractAbiCollection {
                order: 100,
                name: implementation_contract_info.contract_name,
                address: contract_info.implementation,
                abi_id: Some(implementation_abi_id),
                creation_block: None,
                verified: true
            };

            let contract = ContractAbiCollection {
                id:None,
                name: contract_info.contract_name,
                address: contract,
                abi_id: Some(abi_id),
                creation_block: None,
                verified: true,
                is_proxy: contract_info.proxy == "1",
                implementations: vec![implemetation_contract]
            };

            let result = contracts_collection.insert_one(contract, None).await;

            if result.is_err() {
                error!("Failed to insert contract: {:?}", result.err().unwrap());

                continue;
            }
        }
    }
}
