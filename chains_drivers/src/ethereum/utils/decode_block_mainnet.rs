use chrono::{NaiveDateTime, Datelike};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use third_parties::mongo::lib::bronze::blocks::types::Block as MongoBlock;

use crate::ethereum::types::raw::block::Block;
use std::error::Error;

pub fn decode_blocks(blocks: Vec<Block>) -> Result<Vec<MongoBlock>, Box<dyn Error>> {
    
    let blocks = blocks.par_iter().map(|block| {
        let date = NaiveDateTime::from_timestamp_opt(block.timestamp, 0).unwrap();
        let ts = date.timestamp_micros();
        
        MongoBlock {
            timestamp: ts,
            year: date.year() as i16,
            month: date.month() as i8,
            day: date.day() as i8,
            number: block.number,
            hash: block.hash.clone(),
            parent_hash: block.parent_hash.clone(),
            nonce: block.nonce,
            transactions_root: block.transactions_root.clone(),
            state_root: block.state_root.clone(),
            receipts_root: block.receipts_root.clone(),
            miner: block.miner.clone(),
            difficulty: block.difficulty,
            mix_hash: block.mix_hash.clone(),
            extra_data: block.extra_data.clone(),
            logs_bloom: block.logs_bloom.clone(),
            gas_limit: block.gas_limit,
            gas_used: block.gas_used,
            uncles_hash: block.uncles_hash.clone(),
            base_fee_per_gas: block.base_fee_per_gas,
            withdrawals_root: block.withdrawals_root.clone(),
        }
    }).collect();

    Ok(blocks)
}