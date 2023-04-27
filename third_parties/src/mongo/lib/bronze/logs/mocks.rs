use super::types::Log;
use chrono::TimeZone;
use rand::Rng;

pub fn get_mock_logs (ts: &Option<i64>, bn: &Option<i64>, n: Option<i16>) -> Vec<Log> {
    // todo add random number for ts and bn
    let array_len = n.unwrap_or(1);

    let mut blocks = Vec::new();
    
    let mut rng = rand::thread_rng();
    let now = chrono::Utc::now().timestamp_micros();
    let first_ts = chrono::Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap().timestamp_micros();
    let first_bn = 1000000;

    for i in 0..array_len {
        let ts = ts.unwrap_or(rng.gen_range(first_ts..=now));
        
        let bn = bn.unwrap_or(rng.gen_range(first_bn..=18_000_000));
    
        let block = Log {
            timestamp: Some(ts),
            year: Some(2018),
            month:Some(1),
            day: Some(1),
            block_number: bn,
            transaction_hash: Some(String::from("testhash")),
            transaction_index: i64::from(i),
            log_index: i64::from(i),
            address: Some(String::from("thisisamockaddress")),
            data: Some(String::from("thisisamockdata")),
            topics: vec![String::from("thisisamocktopic")],
            block_hash: String::from("thisisamockblockhash"),
            decoded_data: None,
            transaction_log_index: i64::from(i^2),
            removed: false,
            log_type: Some(String::from("thisisamocklogtype")),
        };

        blocks.push(block);
    }

    blocks
}