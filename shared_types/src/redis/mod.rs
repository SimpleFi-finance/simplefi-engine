pub mod abi;

pub enum RedisSupportedLists {
    ABI,
    Blocks,
}

impl std::fmt::Display for RedisSupportedLists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisSupportedLists::ABI => write!(f, "abi"),
            RedisSupportedLists::Blocks => write!(f, "blocks"),
        }
    }
}

impl RedisSupportedLists {
    pub fn get_list_name(&self, chain_symbol: &String) -> String {
        match self {
            RedisSupportedLists::ABI => format!("{}_{}", chain_symbol.to_lowercase(), "abi".to_string()),
            RedisSupportedLists::Blocks => format!("{}_{}", chain_symbol.to_lowercase(), "blocks".to_string()),
        }
    }
}