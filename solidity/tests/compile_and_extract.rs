use std::env;
use std::path::PathBuf;
use solidity::compile_and_extract;
use tokio::runtime::Runtime;
use tracing::info;


#[test]
fn test_compile_and_extract() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let mut contract_path = PathBuf::from(env::current_dir().unwrap());
        contract_path.push("tests");
        contract_path.push("fixtures");
        contract_path.push("ERC20.sol");

        info!("{:?}", contract_path);

        let abi = compile_and_extract::compile_and_extract_abi(&contract_path);

        info!("{:?}", abi);

        assert!(abi.is_ok());
    });

}
