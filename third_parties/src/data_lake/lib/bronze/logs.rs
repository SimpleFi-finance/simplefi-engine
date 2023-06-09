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

use std::{
    fs::File,
};

use super::types::LogsSeries;

pub fn log_file_properties() -> WriterProperties {
    return WriterProperties::builder()
        .set_compression(Compression::ZSTD(ZstdLevel::try_new(4).unwrap()))
        .set_dictionary_enabled(true)
        .set_max_row_group_size(1024 * 1024 * 1024)
        .set_writer_version(WriterVersion::PARQUET_2_0)
        .build();
}

pub fn logs_type() -> Type {
    parse_message_type(
        "
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
    ",
    )
    .unwrap()
}

pub fn write_df_to_file(writer: &mut SerializedFileWriter<File>, data: LogsSeries) -> Result<(), Box<dyn std::error::Error>> {
    println!("Writing to file...");
    let mut row_group_writer = writer.next_row_group().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int64Type>()
        .write_batch(&data.timestamp_data, None, None)
        .unwrap();
    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int32Type>()
        .write_batch(&data.year_data.iter().map(|x| i32::from(*x)).collect::<Vec<i32>>(), None, None)
        .unwrap();
    col_writer.close().unwrap();
    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int32Type>()
        .write_batch(&data.month_data.iter().map(|x| i32::from(*x)).collect::<Vec<i32>>(), None, None)
        .unwrap();
    col_writer.close().unwrap();
    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int32Type>()
        .write_batch(&data.day_data.iter().map(|x| i32::from(*x)).collect::<Vec<i32>>(), None, None)
        .unwrap();
    col_writer.close().unwrap();
    
    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int64Type>()
        .write_batch(&data.block_number_data, None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<ByteArrayType>()
        .write_batch(&data.address_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int32Type>()
        .write_batch(&data.transaction_index_data.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int32Type>()
        .write_batch(&data.log_index_data.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<ByteArrayType>()
        .write_batch(&data.transaction_hash_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
    col_writer
        .typed::<ByteArrayType>()
        .write_batch(&data.topics_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<ByteArrayType>()
        .write_batch(&data.data_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<ByteArrayType>()
        .write_batch(&data.block_hash_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();
    
    col_writer
        .typed::<BoolType>()
        .write_batch(&data.removed_data, None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<ByteArrayType>()
        .write_batch(&data.log_type_data.iter().map(|x| ByteArray::from(x.as_str())).collect::<Vec<ByteArray>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();

    let mut col_writer = row_group_writer.next_column().unwrap().unwrap();

    col_writer
        .typed::<Int32Type>()
        .write_batch(&data.transaction_log_index_data.iter().map(|x| *x as i32).collect::<Vec<i32>>(), None, None)
        .unwrap();

    col_writer.close().unwrap();
    row_group_writer.close().unwrap();

    println!("Finished writing to file!");
    Ok(())
}