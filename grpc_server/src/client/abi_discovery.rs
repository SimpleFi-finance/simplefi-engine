use tonic::transport::Channel;
use tonic::{Request, Response};

use crate::abi_discovery_proto::abi_discovery_service_client::AbiDiscoveryServiceClient;
use crate::abi_discovery_proto::{
    GetAddressesAbiRequest, TrackedAddressesRequest, AddFactoryAddressesRequest,
    AddFactoryAddressesResponse, GetAddressesAbiResponse, TrackedAddressesResponse, GetAddressesAbiJsonResponse,
    GetAbiEventsRequest, GetAbiEventsResponse,
};


pub struct AbiDiscoveryClient {
    client: AbiDiscoveryServiceClient<Channel>,
}

impl AbiDiscoveryClient {
    pub async fn new(address: String) -> Self {
        let client = AbiDiscoveryServiceClient::connect(address).await.unwrap();
        Self { client }
    }

    pub async fn check_tracked_addresses(&mut self, addresses: Vec<String>) -> Response<TrackedAddressesResponse> {
        let request = Request::new(TrackedAddressesRequest { addresses });
        let response = self.client.check_tracked_addresses(request).await.unwrap();

        response
    }

    pub async fn get_addresses_abi(&mut self, addresses: Vec<String>) -> Response<GetAddressesAbiResponse> {
        let request = Request::new(GetAddressesAbiRequest { addresses });
        let response = self.client.get_addresses_abi(request).await.unwrap();

        response
    }

    pub async fn get_addresses_abi_json(&mut self, addresses: Vec<String>) -> Response<GetAddressesAbiJsonResponse> {
        let request = Request::new(GetAddressesAbiRequest { addresses });
        let response = self.client.get_addresses_abi_json(request).await.unwrap();

        response
    }

    pub async fn add_factory_addresses(&mut self, factory_address: String, addresses: Vec<String>) -> Response<AddFactoryAddressesResponse> {
        let request = Request::new(AddFactoryAddressesRequest { factory_address, addresses });
        let response = self.client.add_factory_addresses(request).await.unwrap();

        response
    }

    pub async fn get_abi_events(&mut self, signatures: Vec<String>) -> Response<GetAbiEventsResponse> {
        let request = Request::new(GetAbiEventsRequest { signatures });
        let response = self.client.get_signatures_event(request).await.unwrap();

        response
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{self, bson::doc};
    use redis::AsyncCommands;
    use third_parties::redis::connect;

    async fn clean_check_tracked_addresses_redis() -> redis::RedisResult<()> {
        // connect to local redis and remove keys
        let redis_uri = "redis://localhost:6379/";
        let mut connection = connect(&redis_uri).await.expect("failed to connect to redis");

        // The key of the set and the value to remove.
        let set_key = "abi_discovery_2";
        let value_to_remove_1 = "test_address_1".to_string();
        let value_to_remove_2 = "test_address_2".to_string();

        // Remove the value from the set using the `SREM` command.
        let _: () = connection.srem(set_key, value_to_remove_1).await?;
        let _: () = connection.srem(set_key, value_to_remove_2).await?;


        Ok(())
    }

    async fn clean_check_tracked_addresses_mongo() -> mongodb::error::Result<()> {
        // connect to local mongo and remove keys
        let mongo_uri = "mongodb://localhost:27017/";
        let client = mongodb::Client::with_uri_str(mongo_uri).await.expect("failed to connect to mongo");
        let db = client.database("abi_discovery");
        let collection = db.collection::<mongodb::bson::Document>("abi_discovery");

        let filter = doc! {
            "factory_address": "test_factory_address"
        };

        collection.delete_many(filter, None).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_check_tracked_addresses() {
        let mut client = AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;

        let factory_address= "test_factory_address".to_string();
        let addresses = vec!["test_address_1".to_string(), "test_address_2".to_string()];
        let response = client.add_factory_addresses(factory_address, addresses).await;

        println!("RESPONSE IS ={:?}", response);

        // assert reponse is a Response
        assert!(response.into_inner().success);

        let addresses = vec!["test_address_1".to_string(), "test_address_2".to_string()];
        let response = client.check_tracked_addresses(addresses).await;

        println!("RESPONSE IS={:?}", response);

        assert!(response.into_inner().addresses.len() == 2);

        clean_check_tracked_addresses_redis().await.expect("failed to clean redis");
        clean_check_tracked_addresses_mongo().await.expect("failed to clean mongo");

    }
}




