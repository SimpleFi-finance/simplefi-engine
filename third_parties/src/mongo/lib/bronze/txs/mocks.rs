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
            timestamp: ts,
            year: 2018,
            month: 1,
            day: 1,
            block_number: bn,
            block_hash: String::from("testhash"),
            transaction_index: i as i32,
            to: if i == 1 {String::from("from1")} else {String::from("to")},
            from: if i == 2 {String::from("from1")} else {String::from("from")},
            hash: String::from("testhash"),
            nonce: String::from("nonce"),
            value: String::from("value"),
            gas_price: String::from("gas_price"),
            gas: String::from("gas"),
            input: String::from("input"),
            v: 1,
            r: String::from("r"),
            s: String::from("s"),
        };
        txs.push(tx);
    }
    txs
}