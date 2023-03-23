use clap::Parser;
use shared_types::chains::Chain;
use shared_types::chains::ConnectionType;
use shared_types::chains::SupportedChains::*;
use shared_types::providers::SupportedProviders::*;

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Chain Harvester process to retrieve data from the blockchain")]
#[command(long_about = "Chain Harvester process to retrieve data from the blockchain")]
#[command(next_line_help = true)]
struct Args {
    #[arg(
        short = 'C',
        long = "chain",
        default_value = "ethereum",
        help = "Chain EVM to harvest",
        default_value = "ethereum",
    )]
    chain: String,

    #[arg(
        short = 'P',
        long = "provider",
        help = "Data provider running a node in a chain",
        default_value = "infura",
    )]
    provider: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let chain = match args.chain.as_str() {
        "ethereum" => Mainnet,
        _ => panic!("Chain not supported"),
    };

    println!("{:?}", chain);

    let provider = match args.provider.as_str() {
        "infura" => Infura,
        _ => panic!("Provider not supported"),
    };

    println!("{:?}", provider);

    let chain = Chain::from_chain(chain);

    print!("{:?}", chain.get_node(provider, ConnectionType::RPC).unwrap());

   /*  let node = match chain {
        Mainnet => match provider {
            Infura => "https://mainnet.infura.io/v3/".to_string(),
            _ => panic!("Provider not supported"),
        },
        _ => panic!("Chain not supported"),
    }; */
}
