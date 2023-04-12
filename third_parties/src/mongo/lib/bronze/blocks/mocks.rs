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
            hash: String::from("testhash"),
            parent_hash: String::from("parenthash"),
            uncles_hash: String::from("unclehash"),
            author: String::from("author"),
            state_root: String::from("stateroot"),
            transactions_root: String::from("transactionsroot"),
            receipts_root: String::from("receiptsroot"),
            gas_used: String::from("gasused"),
            gas_limit: String::from("gaslimit"),
            extra_data: String::from("extradata"),
            logs_bloom: String::from("logsbloom"),
            difficulty: String::from("difficulty"),
            total_difficulty: String::from("totaldifficulty"),
            seal_fields: vec![String::from("sealfields")],
            uncles: vec![String::from("uncles")],
            transactions: vec![String::from("tx1")],
            size: String::from("size"),
            mix_hash: String::from("mixhash"),
            nonce: String::from("nonce"),
            base_fee_per_gas: String::from("basefeepergas"),
        };
        blocks.push(block);
    }

    blocks
}