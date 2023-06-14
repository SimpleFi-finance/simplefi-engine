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

use super::{GetSchema, WriteDFToFile, FileProperties};

#[derive(Debug, Clone, Deserialize)]
pub struct TxSeries {
    pub timestamp_data: Vec<i64>,
    pub year_data: Vec<i16>,
    pub month_data: Vec<i8>,
    pub day_data: Vec<i8>,
    pub block_number: Vec<Option<i64>>,
    pub hash: Vec<String>,
    pub transaction_index: Vec<Option<i32>>,
    pub nonce: Vec<String>,
    pub block_hash: Vec<String>,
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub value: Vec<String>,
    pub gas_price: Vec<String>,
    pub gas: Vec<String>,
    pub input: Vec<String>,
    pub v: Vec<i64>,
    pub r: Vec<String>,
    pub s: Vec<String>,
}


impl TxSeries {
    pub fn new () -> Self {
        Self {
            timestamp_data: Vec::new(),
            year_data: Vec::new(),
            month_data: Vec::new(),
            day_data: Vec::new(),
            block_number: Vec::new(),
            hash: Vec::new(),
            transaction_index: Vec::new(),
            nonce: Vec::new(),
            block_hash: Vec::new(),
            from: Vec::new(),
            to: Vec::new(),
            value: Vec::new(),
            gas_price: Vec::new(),
            gas: Vec::new(),
            input: Vec::new(),
            v: Vec::new(),
            r: Vec::new(),
            s: Vec::new(),
        }
    }
}

impl GetSchema for TxSeries {
    fn get_schema() -> Type {
        parse_message_type("
        message schema {
            REQUIRED INT64 timestamp (TIMESTAMP_MICROS);
            REQUIRED INT32 year (INT_16);
            REQUIRED INT32 month (INT_8);
            REQUIRED INT32 day (INT_8);
            REQUIRED INT64 block_number;
            REQUIRED BYTE_ARRAY hash (UTF8);
            REQUIRED INT32 transaction_index;
            REQUIRED BYTE_ARRAY nonce (UTF8);
            REQUIRED BYTE_ARRAY block_hash (UTF8);
            REQUIRED BYTE_ARRAY from (UTF8);
            REQUIRED BYTE_ARRAY to (UTF8);
            REQUIRED BYTE_ARRAY value (UTF8);
            REQUIRED BYTE_ARRAY gas_price (UTF8);
            REQUIRED BYTE_ARRAY gas (UTF8);
            REQUIRED BYTE_ARRAY input (UTF8);
            REQUIRED INT64 v;
            REQUIRED BYTE_ARRAY r (UTF8);
            REQUIRED BYTE_ARRAY s (UTF8);
        }
        ").unwrap()
    }
}

impl WriteDFToFile for TxSeries {
    fn write_to_file(&self, writer: &mut SerializedFileWriter<File>) -> Result<(), Box<dyn std::error::Error>> {
        
        let mut row_group_writer = writer.next_row_group().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
        col_writer
            .typed::<Int64Type>()
            .write_batch(&self.timestamp_data, None, None)
            .unwrap();
        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.year_data.iter().map(|x| i32::from(*x)).collect::<Vec<i32>>(), None, None)
            .unwrap();
        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.month_data.iter().map(|x| i32::from(*x)).collect::<Vec<i32>>(), None, None)
            .unwrap();
        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.day_data.iter().map(|x| i32::from(*x)).collect::<Vec<i32>>(), None, None)
            .unwrap();
        col_writer.close().unwrap();
        
        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int64Type>()
            .write_batch(&self.block_number.iter().map(|x| x.unwrap_or(i64::default())).collect::<Vec<i64>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.hash.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.transaction_index.iter().map(|x| x.unwrap_or(i32::default()) as i32).collect::<Vec<i32>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.nonce.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.block_hash.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
        
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.from.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
        
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.to.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.value.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.gas_price.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
        
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.gas.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.input.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int64Type>()
            .write_batch(&self.v, None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.r.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.s.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        row_group_writer.close().unwrap();

        Ok(())
    }
}

impl FileProperties for TxSeries {
    fn file_properties() -> WriterProperties {
        return WriterProperties::builder()
            .set_compression(Compression::ZSTD(ZstdLevel::try_new(4).unwrap()))
            .set_dictionary_enabled(true)
            .set_max_row_group_size(1024 * 1024 * 1024)
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .build();
    }
}