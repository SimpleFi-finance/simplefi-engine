use chains_types::SupportedChains;

pub struct ProtocolInfo {
    // pub id: SupportedProtocolDrivers,
    pub name: String,
    pub factory_address: String,
    pub chain: SupportedChains,
    pub creation_log_name: String,
    pub protocol_start_year: i32,
}

// TEMP - TODO: replace with official bronze type
pub struct Row {
    timestamp: Option<i64>,
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
    address: Option<String>,
    block_number: Option<i64>,
    block_hash: Option<String>,
    transaction_hash: Option<String>,
    transaction_index: Option<i64>,
    log_index: Option<i64>,
    log_type: String,
    topic1: Option<String>,
    topic2: Option<String>,
    topic3: Option<String>,
    topic4: Option<String>,
    data1: Option<String>,
    data2: Option<String>,
    data3: Option<String>,
    data4: Option<String>,
    data5: Option<String>,
    data6: Option<String>,
    data7: Option<String>,
    data8: Option<String>,
    data9: Option<String>,
    data10: Option<String>,
    removed: Option<bool>,
    tx_log_index: Option<i64>,
}
