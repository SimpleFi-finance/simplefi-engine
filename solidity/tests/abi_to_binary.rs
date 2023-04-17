use std::env;
use std::path::PathBuf;
use solidity::{compile_and_extract, abi_to_binary};
use tokio::runtime::Runtime;
use tracing::{info};


#[test]
fn test_abi_to_binary() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let mut contract_path = PathBuf::from(env::current_dir().unwrap());
        contract_path.push("tests");
        contract_path.push("fixtures");
        contract_path.push("ERC20.sol");

        let abi = compile_and_extract::compile_and_extract_abi(&contract_path);

        info!(abi=?abi, "abi");

        let binary = abi_to_binary::abi_to_binary(&abi.unwrap());

        info!(binary=?binary, "binary");


        assert!(binary.is_ok());


        // test binary_to_abi
        let abi = abi_to_binary::binary_to_abi(&binary.unwrap());

        info!(abi=?abi, "abi");

        assert!(abi.is_ok());

        println!("abi: {:?}", abi.unwrap());
    });

}

