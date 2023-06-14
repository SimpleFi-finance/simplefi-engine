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
        ByteArray, BoolType
    }
};

use crate::data_lake::types::{GetSchema, WriteDFToFile, FileProperties};

#[derive(Debug, Clone, Deserialize)]
pub struct LogsSeries {
    pub timestamp_data: Vec<i64>,
    pub year_data: Vec<i16>,
    pub month_data: Vec<i8>,
    pub day_data: Vec<i8>,
    pub block_number_data: Vec<i64>,
    pub address_data: Vec<String>,
    pub transaction_index_data: Vec<i64>,
    pub log_index_data: Vec<i64>,
    pub transaction_hash_data: Vec<String>,
    pub topics_data: Vec<String>,
    pub data_data: Vec<String>,
    pub block_hash_data: Vec<String>,
    pub removed_data: Vec<bool>,
    pub log_type_data: Vec<String>,
    pub transaction_log_index_data: Vec<i64>,
}

impl LogsSeries {
    pub fn new() -> Self {
        Self {
            timestamp_data: Vec::new(),
            year_data: Vec::new(),
            month_data: Vec::new(),
            day_data: Vec::new(),
            block_number_data: Vec::new(),
            address_data: Vec::new(),
            transaction_index_data: Vec::new(),
            log_index_data: Vec::new(),
            transaction_hash_data: Vec::new(),
            topics_data: Vec::new(),
            data_data: Vec::new(),
            block_hash_data: Vec::new(),
            removed_data: Vec::new(),
            log_type_data: Vec::new(),
            transaction_log_index_data: Vec::new(),
        }
    }
}

impl GetSchema for LogsSeries {
    fn get_schema() -> Type {
        parse_message_type("
        message schema {
            REQUIRED INT64 timestamp (TIMESTAMP_MICROS);
            REQUIRED INT32 year (INT_16);
            REQUIRED INT32 month (INT_8);
            REQUIRED INT32 day (INT_8);
            REQUIRED INT64 block_number;
            REQUIRED BYTE_ARRAY address (UTF8);
            REQUIRED INT32 transaction_index;
            REQUIRED INT32 log_index;
            REQUIRED BYTE_ARRAY transaction_hash (UTF8);
            REQUIRED BYTE_ARRAY topics (UTF8);
            REQUIRED BYTE_ARRAY data;
            REQUIRED BYTE_ARRAY block_hash (UTF8);
            REQUIRED BOOLEAN removed;
            REQUIRED BYTE_ARRAY log_type (UTF8);
            REQUIRED INT32 transaction_log_index;
        }
        ").unwrap()
    }
}

impl WriteDFToFile for LogsSeries {
    fn write_to_file(&self, writer: &mut SerializedFileWriter<File>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Writing to file...");
    
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
            .write_batch(&self.block_number_data, None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.address_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.transaction_index_data.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.log_index_data.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.transaction_hash_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
        
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.topics_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.data_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.block_hash_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
        
        col_writer
            .typed::<BoolType>()
            .write_batch(&self.removed_data, None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&self.log_type_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();

        let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

        col_writer
            .typed::<Int32Type>()
            .write_batch(&self.transaction_log_index_data.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
            .unwrap();

        col_writer.close().unwrap();
        row_group_writer.close().unwrap();

        println!("Finished writing to file!");
        Ok(())
    }
}

impl FileProperties for LogsSeries {
    fn file_properties() -> WriterProperties {
        return WriterProperties::builder()
            .set_compression(Compression::ZSTD(ZstdLevel::try_new(4).unwrap()))
            .set_dictionary_enabled(true)
            .set_max_row_group_size(1024 * 1024 * 1024)
            .set_writer_version(WriterVersion::PARQUET_2_0)
            .build();
    }
}