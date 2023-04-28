use futures::StreamExt;
use mongodb::bson::doc;
use std::{collections::HashMap, error::Error, fs::File, io::Write, path::Path, time::Duration};

use abi_discovery::{
    settings::load_settings,
    types::{ResponseBody, TokensCollection},
};
use third_parties::{mongo::{Mongo, MongoConfig}, broker::{publish_rmq_message, create_rmq_channel}};
#[allow(unused)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mysettings = load_settings().expect("Failed to load settings");

    let mongodb_uri = mysettings.mongodb_uri;
    let mongodb_database_name = mysettings.mongodb_database_name;
    let rabbit_uri = mysettings.rabbit_mq_url.to_string();
    let queue_name = mysettings.rabbit_exchange_name.to_string();
    let exchange_name = format!("{}_exchange", queue_name);
    let routing_key = String::from("abi_discovery");
    let etherscan_keys = mysettings.etherscan_api_keys;

    println!("Setting rabbit mq connection");
    println!("Rabbit URI: {}", rabbit_uri);
    println!("Rabbit Queue Name: {}", queue_name);
    println!("Rabbit Exchange Name: {}", exchange_name);
    println!("Router key: {}", routing_key);
    println!("Etherscan keys: {:?}", etherscan_keys);

    println!("mongodb_uri: {}", mongodb_uri);

    let channel = create_rmq_channel("amqp://localhost:5672/%2f")
        .await
        .expect("Failed to create channel");

    let mongo_config = MongoConfig {
        uri: mongodb_uri.clone(),
        database: mongodb_database_name.to_string(),
    };

    let mongo = Mongo::new(&mongo_config)
        .await
        .expect("Failed to create mongo Client");

    let core_data_collection = mongo.collection::<TokensCollection>("tokens");

    let mut skip = 0;

    println!("Starting to load contract indexes from MongoDB to Redis Set (verify_addresses) ...");

    loop {
        // create query to retrieve all address from chain 1
        let query = doc! {
            "chain": "ethereum",
            // "exported": { "$exists": 0 }
        };

        let options = mongodb::options::FindOptions::builder()
            .limit(500)
            .skip(skip)
            .build();

        let cursor = core_data_collection
            .find(query, options)
            .await
            .expect("Failed to execute find");

        let results = cursor.collect::<Vec<_>>().await;

        if results.is_empty() {
            break;
        }

        println!(
            "Found {} contract indexes. Skipping {}",
            results.len(),
            skip
        );

        for result in results {
            match result {
                Ok(token) => {
                    println!("Token address: {:?}", token.address);

                    let address = token.address;

                    publish_rmq_message(&exchange_name, &routing_key, &address.as_bytes(), &channel)
                                .await
                                .expect("Failed to publish message");

                    // tokio::time::sleep(Duration::from_secs(1)).await;

                    continue;

                    // address = "0x6b175474e89094c44da98b954eedeac495271d0f".to_string();
                    let url =
                        "https://simple-web3-api.herokuapp.com/contracts/get-etherscan-importable";

                    let mut request_body: HashMap<String, String> = HashMap::new();
                    request_body.insert("address".to_string(), address.to_string());

                    // Make a POST request to the website with the input field value
                    let client = reqwest::Client::new();

                    let mut header = reqwest::header::HeaderMap::new();
                    header.insert(
                        "Content-Type",
                        "application/x-www-form-urlencoded".parse().unwrap(),
                    );
                    header.insert("Origin", "https://www.cookbook.dev".parse().unwrap());

                    let res = client.post(url).json(&request_body).send().await?;

                    let response = match res.json::<ResponseBody>().await {
                        Ok(response) => response,
                        Err(e) => {
                            println!("Error: {:?}", e);
                            continue;
                        }
                    };

                    /* let response = res.json::<ResponseBody>().await?; */

                    println!("Response: contracts per chain {:?}", response.imports.len());

                    /* println!("Response: {:?}", response); */

                    let mut abi: Option<String> = None;
                    let mut contract_address: Option<String> = None;
                    let chain_id = 1;

                    if response.contracts.len() > 0 {
                        for contract in response.contracts {
                            if contract.chain_id == chain_id {
                                abi = Some(contract.abi);
                                contract_address = Some(contract.contract_address);
                            }
                        }

                    }

                    if response.imports.len() > 0 {
                        for import in response.imports {
                            if import.chain_id == chain_id {
                                abi = Some(import.abi);
                                contract_address = Some(import.contract_address);
                                // chain_id = contract.chain_id;
                            }
                        }
                    }

                    let abi = match abi {
                        Some(abi) => {
                            abi
                        }
                        None => {
                            println!("No abi found for address: {}", address);

                            // Publish to rabbit mq
                            // we get it from etherscan
                            publish_rmq_message(&exchange_name, &routing_key, &address.as_bytes(), &channel)
                                .await
                                .expect("Failed to publish message");

                            println!("Message published to rabbit mq and waiting 2 seconds...");
                            tokio::time::sleep(Duration::from_secs(2)).await;

                            continue;
                        }
                    };

                    println!("Contract address loaded. trying to save: {:?}", contract_address);
                    // println!("abi: {:?}", abi);

                    let address = contract_address.unwrap();
                    let abi_string = abi.clone();

                    let filename = format!(
                        "E:/simplefi/scrap_jsons/{}-{}.json",
                        address, chain_id
                    );


                    let path = Path::new(&filename);

                    let mut file = match File::create(&path) {
                        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
                        Ok(file) => file,
                    };

                    match file.write_all(abi_string.as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
                        Ok(_) => {
                            println!("successfully wrote to {}", path.display());

                            // Modify mongodb collection with property exported=true
                            let filter = doc! { "address": &address, "chain": "ethereum" };
                            let update = doc! { "$set": { "exported": true } };
                            let options = mongodb::options::UpdateOptions::builder()
                                .upsert(false)
                                .build();

                            let result = core_data_collection
                                .update_one(filter, update, options)
                                .await
                                .expect("Failed to update document.");

                            println!("Updated {} documents.", result.modified_count);

                        }
                    }



                    println!("Saved in file contract address: {} with abi length {} and waiting 2 seconds...", &address, abi_string.len());
                    tokio::time::sleep(Duration::from_secs(2)).await;

                    // println!("Saving contract address: {} with abi length {}", contract_address, abi.len());



                    // loop response.imports
                    /* for import in response.imports {
                        println!(
                            "Import chainid: {}, len: {:?}",
                            import.chainid,
                            import.abi.len()
                        );

                        let filename = format!(
                            "E:/simplefi/scrap_jsons/{}-{}.json",
                            import.implementation, import.chainid
                        );

                        let path = Path::new(&filename);

                        let mut file = match File::create(&path) {
                            Err(why) => panic!("couldn't create {}: {}", path.display(), why),
                            Ok(file) => file,
                        };

                        let data = import.abi.as_str();
                        match file.write_all(data.as_bytes()) {
                            Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
                            Ok(_) => {
                                println!("successfully wrote to {}", path.display());

                                // Modify mongodb collection with property exported=true
                                let filter = doc! { "address": import.implementation, "chain": "ethereum" };
                                let update = doc! { "$set": { "exported": true } };
                                let options = mongodb::options::UpdateOptions::builder()
                                    .upsert(false)
                                    .build();

                                let result = core_data_collection
                                    .update_one(filter, update, options)
                                    .await
                                    .expect("Failed to update document.");

                                println!("Updated {} documents.", result.modified_count);

                            }
                        }
                    }*/
                }
                Err(error_msg) => {
                    println!("Error: {}", error_msg);
                }
            }
        }

        skip += 500;
    }
    /*
    let address = "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string();
    let url = "https://simple-web3-api.herokuapp.com/contracts/get-etherscan-importable";

    let mut request_body: HashMap<String, String> = HashMap::new();
    request_body.insert(
        "address".to_string(),
        "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
    );

    // Make a POST request to the website with the input field value
    let client = reqwest::Client::new();

    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    header.insert("Origin", "https://www.cookbook.dev".parse().unwrap());

    let res = client.post(url).json(&request_body).send().await?;

    let response = res.json::<ResponseBody>().await?;

    println!("Response: contracts per chain {:?}", response.imports.len());

    // loop response.imports
    for import in response.imports {
        println!(
            "Import chainid: {}, len: {:?}",
            import.chainid,
            import.abi.len()
        );

        let filename = format!(
            "E:/simplefi/scrap_jsons/{}-{}.json",
            address, import.chainid
        );

        let path = Path::new(&filename);

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let data = import.abi.as_str();
        match file.write_all(data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
            Ok(_) => println!("successfully wrote to {}", path.display()),
        }
    } */

    Ok(())
}
