use futures::StreamExt;
use bson::oid::ObjectId;
use mongodb::{
    Collection,
    bson::doc,
    error::Error,
    options::FindOptions
};

use super::types::{AbiCollection, ContractAbiCollection};

pub async fn get_abis_by_ids(
    collection: &Collection<AbiCollection>,
    ids: Vec<ObjectId>
) -> Result<Vec<AbiCollection>, Error> {
    let filter = doc! {"_id": {"$in": ids}};

    let mut cursor = collection.find(filter, FindOptions::default()).await?;

    let mut result = Vec::new();

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(doc) => result.push(doc),
            Err(e) => return Err(e.into()),
        }
    }

    Ok(result)
}

pub async fn get_abis_by_hashes(collection: &Collection<AbiCollection>, hashes: Vec<String>) -> Result<Vec<AbiCollection>, Error> {
    let filter = doc! {"abi_hash": {"$in": hashes}};

    let mut cursor = collection.find(filter, FindOptions::default()).await?;

    let mut result = Vec::new();

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(doc) => result.push(doc),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(result)
}

pub async fn get_contracts_by_addresses(
    collection: &Collection<ContractAbiCollection>,
    addresses: Vec<String>
) -> Result<Vec<ContractAbiCollection>, Error> {
    let filter = doc! {"address": {"$in": addresses}};

    let mut cursor = collection.find(filter, FindOptions::default()).await?;

    let mut result = Vec::new();

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(doc) => result.push(doc),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(result)
}
