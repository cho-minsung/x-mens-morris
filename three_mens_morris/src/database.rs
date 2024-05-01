use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection, Database };
use mongodb::results::InsertOneResult;

use futures::stream::StreamExt;

use crate::types::GameHistory;

pub struct TmmClient {
    client: Client,
    db: Database,
    collection: Collection<GameHistory>
}

impl TmmClient {
    pub async fn new() -> TmmClient {
        let uri = "mongodb://localhost:27017";
        let mut client_options = ClientOptions::parse_async(uri).await.unwrap();
        
        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        // Create a new client and connect to the server
        let client = Client::with_options(client_options).unwrap();

        // Send a ping to confirm a successful connection
        client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await
            .unwrap();
        println!("Pinged your deployment. You successfully connected to MongoDB!");

        // Setting up datatbase
        let db = client.database("tmm");

        // Setting up collection
        let collection: Collection<GameHistory> = db.collection("games");

        return TmmClient{ client: client, db: db, collection: collection };
    }

    async fn get_collections(&self) {
        let collection_names = self.db.list_collection_names(None).await.unwrap();
        for collection_name in collection_names {
            println!("{}", collection_name);
        }
    }

    async fn insert_history(&self, doc: GameHistory) {
        let res: InsertOneResult;
        match self.collection.insert_one(doc, None).await {
            Ok(result) => {
                res = result;
                println!("Inserted a document with _id: {}", res.inserted_id);
            },
            Err(_) => {
                println!("Cannot insert a document.")
            },
        };
    }

    async fn get_all_history(&self) {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        while let Some(doc) = cursor.next().await {
            println!("{:?}", doc);
        }
    }
}
