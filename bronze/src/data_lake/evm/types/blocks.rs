use std::fs::File;

use serde::Deserialize;
use parquet::{
    file::{properties::{WriterProperties, WriterVersion}, writer::SerializedFileWriter}, 
    basic::{Compression, ZstdLevel}, 
    schema::{
        parser::parse_message_type,
        types::Type
    }, 
    data_type::{
        Int64Type, 
        Int32Type, 
        ByteArrayType, 
        ByteArray
    }
};


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BlockSeries {
    pub timestamp: Vec<i64>, 
    pub year: Vec<i16>, 
    pub month: Vec<i8>, 
    pub day: Vec<i8>, 
    pub number: Vec<i64>, 
    pub hash: Vec<String>, 
    pub parent_hash: Vec<String>, 
    pub uncles_hash: Vec<String>, 
    pub miner: Vec<String>, 
    pub state_root: Vec<String>, 
    pub transactions_root: Vec<String>, 
    pub receipts_root: Vec<String>, 
    pub gas_used: Vec<String>, 
    pub gas_limit: Vec<String>, 
    pub extra_data: Vec<String>, 
    pub logs_bloom: Vec<String>, 
    pub difficulty: Vec<String>, 
    pub total_difficulty: Vec<String>, 
    pub seal_fields: Vec<String>, 
    pub uncles: Vec<String>, 
    pub size: Vec<String>, 
    pub mix_hash: Vec<String>, 
    pub nonce: Vec<String>, 
    pub base_fee_per_gas: Vec<String>, 
}


impl BlockSeries {
    pub fn new() -> Self {
        Self {
            timestamp: Vec::new(), 
            year: Vec::new(), 
            month: Vec::new(), 
            day: Vec::new(), 
            number: Vec::new(), 
            hash: Vec::new(), 
            parent_hash: Vec::new(), 
            uncles_hash: Vec::new(), 
            miner: Vec::new(), 
            state_root: Vec::new(), 
            transactions_root: Vec::new(), 
            receipts_root: Vec::new(), 
            gas_used: Vec::new(), 
            gas_limit: Vec::new(), 
            extra_data: Vec::new(), 
            logs_bloom: Vec::new(), 
            difficulty: Vec::new(), 
            total_difficulty: Vec::new(), 
            seal_fields: Vec::new(), 
            uncles: Vec::new(), 
            size: Vec::new(), 
            mix_hash: Vec::new(), 
            nonce: Vec::new(), 
            base_fee_per_gas: Vec::new(),
        }
    }

    pub fn get_schema() -> Type {
        parse_message_type("
            message schema {
                REQUIRED INT64 timestamp (TIMESTAMP_MICROS);
                REQUIRED INT32 year (INT_16);
                REQUIRED INT32 month (INT_8);
                REQUIRED INT32 day (INT_8);
                REQUIRED INT64 number;
                REQUIRED BYTE_ARRAY hash (UTF8);
                REQUIRED BYTE_ARRAY parent_hash (UTF8);
                REQUIRED BYTE_ARRAY uncles_hash (UTF8);
                REQUIRED BYTE_ARRAY author (UTF8);
                REQUIRED BYTE_ARRAY state_root (UTF8);
                REQUIRED BYTE_ARRAY transactions_root (UTF8);
                REQUIRED BYTE_ARRAY receipts_root (UTF8);
                REQUIRED BYTE_ARRAY gas_used (UTF8);
                REQUIRED BYTE_ARRAY gas_limit (UTF8);
                REQUIRED BYTE_ARRAY extra_data (UTF8);
                REQUIRED BYTE_ARRAY logs_bloom (UTF8);
                REQUIRED BYTE_ARRAY difficulty (UTF8);
                REQUIRED BYTE_ARRAY total_difficulty (UTF8);
                REQUIRED BYTE_ARRAY seal_fields (UTF8);
                REQUIRED BYTE_ARRAY uncles (UTF8);
                REQUIRED BYTE_ARRAY transactions (UTF8);
                REQUIRED BYTE_ARRAY size (UTF8);
                REQUIRED BYTE_ARRAY mix_hash (UTF8);
                REQUIRED BYTE_ARRAY nonce (UTF8);
                REQUIRED BYTE_ARRAY base_fee_per_gas (UTF8);
            }
        ").unwrap()
    }

    pub fn write_to_file(&self, writer: &mut SerializedFileWriter<File>) -> Result<(), Box<dyn std::error::Error>> {
        let mut row_group_writer = writer.next_row_group().unwrap();
    
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<Int64Type>()
            .write_batch(&self.timestamp, None, None)
            .unwrap();
        col_writer.close().unwrap();
        println!("Wrote timestamp column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.year.iter().map(|x| i32::from(*x)).collect::<Vec<i32>>(), None, None)
            .unwrap();
        col_writer.close().unwrap();
        println!("Wrote year column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.month.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
            .unwrap();
        col_writer.close().unwrap();
        println!("Wrote month column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.day.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
            .unwrap();
        col_writer.close().unwrap();
        println!("Wrote day column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<Int64Type>()
            .write_batch(&self.number, None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote number column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.hash.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote hash column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.parent_hash.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote parent_hash column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.uncles_hash.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote uncles_hash column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.miner.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote author column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.state_root.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote state_root column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.transactions_root.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote transactions_root column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.receipts_root.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
    
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.gas_used.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote gas_used column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.gas_limit.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote gas_limit column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.extra_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote extra_data column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.logs_bloom.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote logs_bloom column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.difficulty.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote difficulty column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.total_difficulty.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote total_difficulty column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.seal_fields.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote seal_fields column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.uncles.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote uncles column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer.close().unwrap();
        println!("Wrote transactions column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.size.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote size column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.mix_hash.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote mix_hash column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.nonce.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        println!("Wrote nonce column");
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.base_fee_per_gas.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();
    
        col_writer.close().unwrap();
        row_group_writer.close().unwrap();
    
        Ok(())
    }

    pub fn file_properties() -> WriterProperties {
        return WriterProperties::builder()
            .set_compression(Compression::ZSTD(ZstdLevel::try_new(4).unwrap()))
            .set_dictionary_enabled(true)
            .set_max_row_group_size(1024 * 1024 * 1024)
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .build();
    }
}

