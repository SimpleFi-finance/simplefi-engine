use serde::{Serialize, Deserialize};
use mongo_types::Mongo;

pub async fn save_to_db<R>(
    items: Vec<R>,
    db: &Mongo,
    collection_name: String,
) where
    for<'a> R: Deserialize<'a> + Serialize,
{   
    if items.len() == 0 {
        return;
    }

    let collection = db.collection::<R>(&collection_name);

    collection.insert_many(items, None).await.unwrap();
}