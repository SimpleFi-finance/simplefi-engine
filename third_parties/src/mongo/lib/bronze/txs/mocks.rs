use super::types::Tx;
use chrono::TimeZone;
use rand::Rng;

pub fn get_mock_tx (ts: &Option<i64>, bn: &Option<i64>, n: Option<i16>) -> Vec<Tx> {
    // todo add random number for ts and bn
    let array_len = n.unwrap_or(1);

    let mut txs = Vec::new();
    
    let mut rng = rand::thread_rng();
    let now = chrono::Utc::now().timestamp_micros();
    let first_ts = chrono::Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap().timestamp_micros();
    let first_bn = 1000000;

    for i in 0..array_len {
        let ts = ts.unwrap_or(rng.gen_range(first_ts..=now));
        
        let bn = bn.unwrap_or(rng.gen_range(first_bn..=18_000_000));
    
        let tx = Tx {
            timestamp: Some(ts),
            year: Some(2018),
            month: Some(1),
            day: Some(1),
            block_number: bn,
            block_hash: Some(String::from("testhash")),
            transaction_index: i as i32,
            to: if i == 1 {Some(String::from("from1"))} else {Some(String::from("to"))},
            from: if i == 2 {Some(String::from("from1"))} else {Some(String::from("from"))},
            hash: Some(String::from("testhash")),
            nonce: Some(String::from("nonce")),
            value: Some(String::from("value")),
            gas_price: 2,
            gas: 1,
            input: Some(String::from("input")),
            v: 1,
            r: Some(String::from("r")),
            s: Some(String::from("s")),
        };
        txs.push(tx);
    }
    txs
}