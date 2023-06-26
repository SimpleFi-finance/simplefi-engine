use bson::oid::ObjectId;
use hex;
use mongodb::{bson::doc, error::Error as Err, Collection};
use ring::digest::{digest, SHA256};

use super::types::{AbiCollection, ContractAbiCollection};

pub fn hash_abi(abi: &str) -> String {
    let result = digest(&SHA256, abi.as_bytes());
    hex::encode(result.as_ref())
}

pub async fn insert_abi(
    abi_collection: &Collection<AbiCollection>,
    abi: &str,
    is_proxy: &bool,
    standard: &u32,
) -> Result<ObjectId, Err> {
    let abi_hash = hash_abi(abi);

    let abi_result = abi_collection
        .find_one(
            doc! {
                "abi_hash": abi_hash.clone().to_string()
            },
            None,
        )
        .await?;

    match abi_result {
        Some(document) => {
            return Ok(document.id.unwrap());
        }
        None => {
            let abi = AbiCollection {
                id: None,
                abi: abi.to_string(),
                abi_hash: abi_hash.clone().to_string(),
                is_proxy: is_proxy.clone(),
                standard: standard.clone(),
            };

            let insert_result = abi_collection.insert_one(abi.clone(), None).await?;

            return Ok(insert_result.inserted_id.as_object_id().unwrap());
        }
    }
}

pub async fn insert_contract(
    contract_collection: &Collection<ContractAbiCollection>,
    contract: ContractAbiCollection,
) -> Result<ContractAbiCollection, Err> {
    let contract_result = contract_collection
        .find_one(
            doc! {
                "address": contract.address.clone().to_string()
            },
            None,
        )
        .await?;

    match contract_result {
        Some(document) => {
            Ok(document)
        }
        None => {
            let insert_result = contract_collection
                .insert_one(contract.clone(), None)
                .await?;

            let mut return_document = contract;

            return_document.id = insert_result.inserted_id.as_object_id();

            Ok(return_document)
        }
    }
}


pub async fn insert_abis(
    abi_collection: &Collection<AbiCollection>,
    abis: Vec<AbiCollection>,
) -> Result<Vec<AbiCollection>, Err> {
    let mut result = Vec::new();

    for abi in abis {
        let abi_hash = hash_abi(&abi.abi);

        let abi_result = abi_collection
            .find_one(
                doc! {
                    "abi_hash": abi_hash.clone().to_string()
                },
                None,
            )
            .await?;

        match abi_result {
            Some(document) => {
                result.push(document);
            }
            None => {
                let mut abi = abi;

                abi.abi_hash = abi_hash.clone().to_string();

                let insert_result = abi_collection.insert_one(abi.clone(), None).await?;

                let mut return_document = abi;

                return_document.id = insert_result.inserted_id.as_object_id();

                result.push(return_document);
            }
        }
    }

    Ok(result)
}

pub async fn insert_contracts(
    contract_collection: &Collection<ContractAbiCollection>,
    contracts: Vec<ContractAbiCollection>,
) -> Result<Vec<ContractAbiCollection>, Err> {
    let mut result = Vec::new();

    for contract in contracts {
        let contract_result = contract_collection
            .find_one(
                doc! {
                    "address": contract.address.clone().to_string()
                },
                None,
            )
            .await?;

        match contract_result {
            Some(document) => {
                result.push(document);
            }
            None => {
                let insert_result = contract_collection
                    .insert_one(contract.clone(), None)
                    .await?;

                let mut return_document = contract;

                return_document.id = insert_result.inserted_id.as_object_id();

                result.push(return_document);
            }
        }
    }

    Ok(result)
}
