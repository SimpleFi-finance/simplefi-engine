use tonic::transport::Channel;

pub mod abi_discovery_proto {
    tonic::include_proto!("abi_discovery_proto");
}

use abi_discovery_proto::abi_discovery_service_client::{ AbiDiscoveryServiceClient };
use abi_discovery_proto::{TrackedAddressesRequest };

// Create a main function that will call the server
// and print the response
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a channel to the server
    let channel = Channel::from_static("http://[::1]:50051")
    // let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    // Create a client with the channel
    let mut client = AbiDiscoveryServiceClient::new(channel);

    // Create a request with the data we want to send to the server
    let request = tonic::Request::new(TrackedAddressesRequest {
        addresses:  vec!["0x0000000000".to_string(), "0x0000000001".to_string()],
    });

    // Send the request to the server and wait for a response
    let response = client.check_tracked_addresses(request).await?;

    // Print the response we got from the server
    println!("RESPONSE={:?}", response);

    Ok(())
}

