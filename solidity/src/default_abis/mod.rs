use log::info;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FactoryAbi {
    pub abi: String,
    pub address: String,
}

pub async fn get_factory_abis() -> HashMap<u32, FactoryAbi> {
    let mut factory_abis: HashMap<u32, FactoryAbi> = HashMap::new();

    factory_abis.insert(
        10,
        FactoryAbi {
            abi: include_str!("uniswap_v2_factory_abi.json").to_string(),
            address: "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f".to_string(),
        },
    );

    factory_abis.insert(
        50,
        FactoryAbi {
            abi: include_str!("sushiswap_v2_factory_abi.json").to_string(),
            address: "0xc0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac".to_string(),
        },
    );

    info!("factory_abis: {:?}", factory_abis);

    factory_abis
}

pub async fn get_default_market_abis() -> HashMap<u32, String> {
    let mut default_market_abis: HashMap<u32, String> = HashMap::new();

    default_market_abis.insert(
        11,
        include_str!("uniswap_v2_default_market_abi.json").to_string(),
    );
    default_market_abis.insert(
        51,
        include_str!("sushiswap_v2_default_market_abi.json").to_string(),
    );

    default_market_abis
}

pub fn get_factory_market_index(address: &String) -> u32 {
    match address.as_str() {
        "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f" => 11 as u32,
        "0xc0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac" => 51 as u32,
        _ => 0,
    }
}

/*
#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

     #[test]
      fn test_get_factory_abis() {
        let factory_abis = get_factory_abis();

        assert!(factory_abis.await.len() > 0);

        assert!(factory_abis.contains_key(&10));
        assert!(factory_abis.contains_key(&50));

        assert_eq!(factory_abis.get(&10).unwrap().abi, include_str!("uniswap_v2_factory_abi.json"));
        assert_eq!(factory_abis.get(&50).unwrap().abi, include_str!("sushiswap_v2_factory_abi.json"));
    }

    #[test]
    fn test_get_default_market_abis() {
        let default_market_abis = get_default_market_abis();

        assert!(default_market_abis.len() > 0);

        assert!(default_market_abis.contains_key(&11));
        assert!(default_market_abis.contains_key(&51));

        assert_eq!(default_market_abis.get(&11).unwrap(), include_str!("uniswap_v2_default_market_abi.json"));
        assert_eq!(default_market_abis.get(&51).unwrap(), include_str!("sushiswap_v2_default_market_abi.json"));
    }

    #[test]
    fn test_enum_default_market_abis() {
        let default_market_abis = get_default_market_abis();

        assert!(default_market_abis.len() > 0);

        let uniswap_v2 = DefaultMarketAbi::UniswapV2 as u32;
        let sushiswap_v2 = DefaultMarketAbi::SushiswapV2 as u32;


        assert!(default_market_abis.contains_key(&uniswap_v2));
        assert!(default_market_abis.contains_key(&sushiswap_v2));

        assert_eq!(default_market_abis.get(&uniswap_v2).unwrap(), include_str!("uniswap_v2_default_market_abi.json"));
        assert_eq!(default_market_abis.get(&sushiswap_v2).unwrap(), include_str!("sushiswap_v2_default_market_abi.json"));
    }
}
*/
