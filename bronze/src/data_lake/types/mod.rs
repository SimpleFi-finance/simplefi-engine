use std::fs::File;

use parquet::{
    schema::{
        types::Type
    }, file::{writer::SerializedFileWriter, properties::WriterProperties}, 
};

pub trait GetSchema {
    fn get_schema() -> Type;
}

pub trait WriteDFToFile {
    fn write_to_file(&self, writer: &mut SerializedFileWriter<File>)  -> Result<(), Box<dyn std::error::Error>>;
}

pub trait FileProperties {
    fn file_properties() -> WriterProperties;
}