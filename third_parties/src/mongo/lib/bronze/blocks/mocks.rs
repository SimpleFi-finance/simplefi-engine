use super::types::Block;
use chrono::TimeZone;
use rand::Rng;

pub fn get_mock_block (ts: &Option<i64>, bn: &Option<i64>, n: Option<i16>) -> Vec<Block> {
    // todo add random number for ts and bn
    let array_len = n.unwrap_or(1);

    let mut blocks = Vec::new();
    
    let mut rng = rand::thread_rng();
    let now = chrono::Utc::now().timestamp_micros();
    let first_ts = chrono::Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap().timestamp_micros();
    let first_bn = 1000000;

    for _ in 0..array_len {
        let ts = ts.unwrap_or(rng.gen_range(first_ts..=now));
        
        let bn = bn.unwrap_or(rng.gen_range(first_bn..=18_000_000));
    
        let block = Block {
            timestamp: ts,
            year: 2018,
            month: 1,
            day: 1,
            number: bn,
            hash: Some(String::from("testhash")),
            parent_hash: Some(String::from("parenthash")),
            state_root: Some(String::from("stateroot")),
            transactions_root: Some(String::from("transactionsroot")),
            receipts_root: Some(String::from("receiptsroot")),
            gas_used: 10,
            gas_limit: 5,
            extra_data: Some(String::from("extradata")),
            logs_bloom: Some(String::from("logsbloom")),
            difficulty: 3,
            mix_hash: Some(String::from("mixhash")),
            nonce: 3,
            base_fee_per_gas: 2,
            miner: Some(String::from("miner")),
            uncles_hash: Some(String::from("sha3uncles")),
            withdrawals_root: Some(String::from("withdrawalsroot")),
        };
        blocks.push(block);
    }

    blocks
}