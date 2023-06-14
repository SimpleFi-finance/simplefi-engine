use ethabi::Contract;
use log::debug;
use std::collections::HashMap;

use shared_types::abi_discovery::AbiStandards;

///
/// Returns the ABI standard of the contract
///
/// # Arguments
///
/// * `abi` - The ABI of the contract
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::abis::get_abi_standard;
///
/// let abi = r#"
/// [
///    {...}
/// ]
/// "#;
///
/// let abi_standard = get_abi_standard(abi);
///
/// assert_eq!(abi_standard, AbiStandards::ERC20);
///
/// ```
///
/// # Panics
///
/// Panics if the ABI is invalid
///
/// # Errors
///
/// Returns an error if the ABI is invalid
///
pub fn get_abi_standard(abi: &str) -> AbiStandards {
    let contract_abi = Contract::load(abi.as_bytes());

    if contract_abi.is_err() {
        debug!("Failed to load contract ABI");

        return AbiStandards::Custom;
    }

    let contract_abi = contract_abi.unwrap();

    if is_erc20(&contract_abi) {
        debug!("The ABI contains more than half ERC-20 functions and events.");
        return AbiStandards::ERC20;
    }

    if is_erc721(&contract_abi) {
        debug!("The ABI contains more than half ERC-721 functions and events.");
        return AbiStandards::ERC721;
    }

    if is_erc777(&contract_abi) {
        debug!("The ABI contains more than half ERC-777 functions and events.");
        return AbiStandards::ERC777;
    }

    if is_erc1155(&contract_abi) {
        debug!("The ABI contains more than half ERC-1155 functions and events.");
        return AbiStandards::ERC1155;
    }

    AbiStandards::Custom
}

///
/// Returns true if the contract is ERC-20
///
fn is_erc20(abi_contract: &Contract) -> bool {
    let mut erc20_signatures: HashMap<&str, f32> = HashMap::new();
    erc20_signatures.insert("totalSupply", 1.0);
    erc20_signatures.insert("balanceOf", 1.0);
    erc20_signatures.insert("transfer", 1.0);
    erc20_signatures.insert("transferFrom", 1.0);
    erc20_signatures.insert("approve", 1.0);
    erc20_signatures.insert("allowance", 1.0);
    erc20_signatures.insert("Transfer", 0.5);
    erc20_signatures.insert("Approval", 0.5);
    let max = 7.0;

    let mut score = 0.0;

    for function in abi_contract.functions() {
        if erc20_signatures.contains_key(function.name.as_str()) {
            score += erc20_signatures.get(function.name.as_str()).unwrap();
        }
    }

    for event in abi_contract.events() {
        if erc20_signatures.contains_key(event.name.as_str()) {
            score += erc20_signatures.get(event.name.as_str()).unwrap();
        }
    }

    debug!("ERC20 score: {}", score);

    score >= max / 2.0
}

///
/// Returns true if the contract is ERC-721
///
fn is_erc721(abi_contract: &Contract) -> bool {
    let mut signatures: HashMap<&str, f32> = HashMap::new();
    signatures.insert("ownerOf", 0.5);
    signatures.insert("balanceOf", 1.0);
    signatures.insert("transfer", 1.0);
    signatures.insert("transferFrom", 1.0);
    signatures.insert("approve", 1.0);
    signatures.insert("safeTransferFrom", 1.0);
    signatures.insert("Transfer", 0.5);
    signatures.insert("Approval", 0.5);
    signatures.insert("ApprovalForAll", 0.5);


    let max = 7.0;

    let mut score = 0.0;

    for function in abi_contract.functions() {
        if signatures.contains_key(function.name.as_str()) {
            score += signatures.get(function.name.as_str()).unwrap();
        }
    }

    for event in abi_contract.events() {
        if signatures.contains_key(event.name.as_str()) {
            score += signatures.get(event.name.as_str()).unwrap();
        }
    }

    debug!("ERC721 score: {}", score);

    score >= max / 2.0
}

///
/// Returns true if the contract is ERC-777
///
fn is_erc777(abi_contract: &Contract) -> bool {
    let mut signatures: HashMap<&str, f32> = HashMap::new();
    signatures.insert("name", 0.5);
    signatures.insert("symbol", 0.5);
    signatures.insert("granularity", 0.5);
    signatures.insert("totalSupply", 1.0);
    signatures.insert("balanceOf", 1.0);
    signatures.insert("send", 1.0);
    signatures.insert("burn", 0.5);
    signatures.insert("defaultOperators", 0.5);
    signatures.insert("authorizeOperator", 0.5);
    signatures.insert("revokeOperator", 0.5);
    signatures.insert("isOperatorFor", 0.5);
    signatures.insert("isOperatorFor", 0.5);
    signatures.insert("operatorSend", 0.5);
    signatures.insert("operatorBurn", 0.5);
    signatures.insert("Transfer", 1.0);
    signatures.insert("Burned", 0.5);
    signatures.insert("AuthorizedOperator", 0.5);
    signatures.insert("RevokedOperator", 0.5);

    let max = 11.0;
    let mut score = 0.0;

    for function in abi_contract.functions() {
        if signatures.contains_key(function.name.as_str()) {
            score += signatures.get(function.name.as_str()).unwrap();
        }
    }

    for event in abi_contract.events() {
        if signatures.contains_key(event.name.as_str()) {
            score += signatures.get(event.name.as_str()).unwrap();
        }
    }

    debug!("ERC777 score: {}", score);

    score >= max / 2.0
}

///
/// Returns true if the contract is ERC-1155
///
fn is_erc1155(abi_contract: &Contract) -> bool {
    let mut signatures: HashMap<&str, f32> = HashMap::new();
    signatures.insert("balanceOf", 1.0);
    signatures.insert("balanceOfBatch", 1.0);
    signatures.insert("setApprovalForAll", 0.5);
    signatures.insert("isApprovedForAll", 0.5);
    signatures.insert("safeTransferFrom", 1.0);
    signatures.insert("safeBatchTransferFrom", 1.0);
    signatures.insert("mint", 1.0);
    signatures.insert("burn", 0.5);
    signatures.insert("batchMint", 0.5);
    signatures.insert("batchBurn", 0.5);
    signatures.insert("TransferSingle", 1.0);
    signatures.insert("TransferBatch", 1.0);
    signatures.insert("ApprovalForAll", 0.5);

    let max = 11.0;
    let mut score = 0.0;

    for function in abi_contract.functions() {
        if signatures.contains_key(function.name.as_str()) {
            score += signatures.get(function.name.as_str()).unwrap();
        }
    }

    for event in abi_contract.events() {
        if signatures.contains_key(event.name.as_str()) {
            score += signatures.get(event.name.as_str()).unwrap();
        }
    }

    debug!("ERC777 score: {}", score);

    score >= max / 2.0
}
