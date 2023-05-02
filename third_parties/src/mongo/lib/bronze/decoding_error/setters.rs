use settings::load_settings;

use crate::mongo::Mongo;

use super::types::DecodingError;

pub async fn save_decoding_error(db: &Mongo, errors: Vec<DecodingError>) {

    let global_settings = load_settings().unwrap();

    let decoding_error_collection = db.collection::<DecodingError>(&global_settings.decoding_error_bronze_collection_name);

    decoding_error_collection.insert_many(errors, None).await.unwrap();
}