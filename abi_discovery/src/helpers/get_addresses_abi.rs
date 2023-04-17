use crate::helpers::check_tracked_addresses;


pub async fn get_addresses_abi(
     addresses: &Vec<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let tracked_address = check_tracked_addresses(addresses).await?;

    Ok(tracked_address)
}

