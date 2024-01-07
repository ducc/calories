use chrono::NaiveDate;
use mongodb::{bson::doc, options::FindOneAndReplaceOptions, Collection};
use serde::{Deserialize, Serialize};

use crate::entries::EntriesResponse;

pub struct Client {
    client: mongodb::Client,
    database: mongodb::Database,
}

impl Client {
    pub async fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let uri = std::env::var("MONGODB_CONNECTION_URI")?;
        let database_name = std::env::var("MONGODB_DATABASE_NAME")?;

        let client = mongodb::Client::with_uri_str(uri).await?;
        let database = client.database(&database_name);

        Ok(Self { client, database })
    }

    pub async fn insert_entries(
        &self,
        date: NaiveDate,
        entries: EntriesResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Serialize, Deserialize)]
        struct EntriesWrapper {
            #[serde(rename = "_id")]
            date: NaiveDate,
            #[serde(rename = "data")]
            entries: EntriesResponse,
        }

        let collection: Collection<EntriesWrapper> = self.database.collection("nutracheck_entries");

        // let _ = collection.update_one(EntriesWrapper {
        //     date, entries
        // }, None).await?;

        let wrapped = EntriesWrapper { date, entries };

        collection
            .find_one_and_replace(
                doc! { "_id": date.to_string(), },
                wrapped,
                FindOneAndReplaceOptions::builder().upsert(true).build(),
            )
            .await?;

        Ok(())
    }
}

//     let client = mongodb::Client::with_uri_str(uri).await?;

//     let database = client.database("calories");

//     let collection: Collection<Document> = database.collection("nutracheck");

//     let doc = doc! {
//         "name": "bob",
//         "age": 23,
//     };

//     let result = collection.insert_one(doc, None).await?;

//     println!("{:?}", result);

//     Ok(())
// }
