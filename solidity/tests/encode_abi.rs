use std::env;
use std::path::PathBuf;
use solidity::{compile_and_extract, encode_abi};
use tokio::runtime::Runtime;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;


#[test]
fn test_abi_to_binary() {
    let rt = Runtime::new().unwrap();

    setup_test_tracing();

    rt.block_on(async {
        let mut contract_path = PathBuf::from(env::current_dir().unwrap());
        contract_path.push("tests");
        contract_path.push("fixtures");
        contract_path.push("ERC20.sol");

        let abi = compile_and_extract::compile_and_extract_abi(&contract_path);
        info!(abi=?abi, "abi");

        let encoded_abi = encode_abi::encode_abi(&abi.unwrap());
        info!(encoded_abi=?encoded_abi, "encoded_abi");
    });
}

pub fn setup_test_tracing() {
    // You can use environment variables to control the tracing level.
    // For example, you can run your tests with `RUST_LOG=trace cargo test` to show `trace` and `debug` messages.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
