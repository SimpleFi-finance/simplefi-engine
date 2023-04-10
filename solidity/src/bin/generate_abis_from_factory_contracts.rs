use bson::{doc, Binary};
use std::fs::read_dir;
use tokio;

use shared_types::{abi::Abi, mongo::abi::AbiCollection };
use third_parties::mongo::{Mongo, MongoConfig};

// create main function
#[tokio::main]
async fn main() {
    // Create a mongo connection with Mongo helper from shared_types
    let mongo_config = MongoConfig {
        uri: "mongodb://localhost:27017".to_string(),
        database: "abi_discovery_2".to_string(),
    };

    // print!("mongoConfig: {:?}", mongo_config);

    // Create a new MongoDB client
    let mongo = Mongo::new(&mongo_config).await.expect("Failed to create mongo Client");
    let abi_collection = mongo.database.collection::<AbiCollection>("abis");
    /* let factory_collection = mongo
        .database
        .collection::<FactoryAbiCollection>("factory-abi"); */

    // We need to get the list of folders in a directory
    let files = read_dir("E:\\solidity\\factory_contracts")
        .expect("Failed to read root directory with factory contracts");

    // Iterate over the directory entries
    for entry in files {
        // Get the entry path
        let file_path = entry.unwrap().path();

        // Check if the file path is a file
        if file_path.is_file() {
            let filedata = std::fs::read_to_string(&file_path)
                            .expect("Failed to read metadata.json");

            let filedata_json = serde_json::from_str::<serde_json::Value>(&filedata)
                            .expect("Failed to parse metadata as JSON");

            let name = filedata_json
                .get("name")
                .expect("No name property with ABI definition");

            let address = filedata_json
                .get("address")
                .expect("No name property with ABI definition");

            let abi_string = filedata_json
                .get("abi")
                .expect("No name property with ABI definition")
                .to_string();

            println!("filedata_name: {:?}", name);
            println!("filedata_address: {:?}", address);
            // println!("filedata_abi: {:?}", abi);
            println!("");

            let mut abi: Vec<Abi> = serde_json::from_str(&abi_string).unwrap();

            // sort abi
            Abi::sort_abi_elements(&mut abi);

            // sort parameters
            for abi in &mut abi {
                abi.sort_parameters();
            }

            let abi_bytecode = bincode::serialize(&abi).unwrap();

            // Check if there's any document with the 'abi' field equal to the ABI bytecode.
            let abi_binary = Binary {
                subtype: bson::spec::BinarySubtype::Generic,
                bytes: abi_bytecode,
            };

            let filter = doc! { "abi": &abi_binary };

            let result = abi_collection
                .find_one(filter, None)
                .await
                .expect("Failed to execute find");

            match result {
                Some(_) => {
                    println!("Found a repeated ABI for document: {:?}", &name);
                }
                None => {
                    println!("No document found. finding index on last document");
                }
            }

            // Currently we are tracking Uniswap V2 factory contracts and Sushiswap factory contracts and we already have them in the database.
        }
    }
}


