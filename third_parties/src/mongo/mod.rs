
use mongodb::{ Client, Collection, Database };
pub mod lib;
// Define a struct to hold the MongoDB configuration
#[derive(Clone, Debug)]
pub struct MongoConfig {
    pub uri: String,
    pub database: String,
}

// Define a struct to hold the MongoDB client
#[derive(Clone, Debug)]
pub struct Mongo {
    pub client: Client,
    pub database: Database,
}

// Implement the Mongo struct
impl Mongo {
    // Create a new instance of the Mongo struct
    pub async fn new(config: &MongoConfig) -> Result<Self, mongodb::error::Error> {
        // Create a new MongoDB client
        let client = Client::with_uri_str(&config.uri).await?;

        // Get a handle to the database
        let database = client.database(&config.database);

        // Return the Mongo struct
        Ok(Self { client, database })
    }

    // Get a handle to a MongoDB collection
    pub fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: serde::de::DeserializeOwned + serde::Serialize,
    {
        self.database.collection(name)
    }
}

// create a test for Mongo Struct which connects to a localhost mongo
#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_mongo() {
        // Create a new MongoDB configuration
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();

        // Get a handle to the "users" collection
        let users = mongo.collection::<User>("users");

        // Insert a new user
        let inserted = users
            .insert_one(
                User {
                    name: "John Doe".to_string(),
                    age: 42,
                },
                None,
            )
            .await
            .unwrap();

        print!("{:?}", &inserted);

        // Find the user we just inserted
        let user = users
            .find_one(doc! { "name": "John Doe" }, None)
            .await
            .unwrap()
            .unwrap();

        print!("{:?}", &user);

        // Assert that the user's name is correct
        assert_eq!(user.name, "John Doe");

        // Assert that the user's age is correct
        assert_eq!(user.age, 42);

        // Drop the "users" collection
        users.drop(None).await.unwrap();

    }

    // Define a struct to hold user information
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct User {
        name: String,
        age: u8,
    }
}

