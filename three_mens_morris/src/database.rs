use mongodb::{ bson::{self, doc}, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection, Database };
use mongodb::results::InsertOneResult;
use mongodb::error::Result as MongoResult;
use bson::oid::ObjectId;

use uuid::Uuid;

use futures::stream::StreamExt;

use serde_json::json;

use crate::types::{GameHistory, OngoingGame};

pub struct TmmDbClient {
    client: Client,
    db: Database,
    game_history: Collection<GameHistory>,
    ongoing_games: Collection<OngoingGame>,
}

impl TmmDbClient {
    pub async fn new() -> TmmDbClient {
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
        let game_history: Collection<GameHistory> = db.collection("game_history");
        let ongoing_games: Collection<OngoingGame> = db.collection("ongoing_games");

        return TmmDbClient{ client: client, db: db, game_history: game_history, ongoing_games: ongoing_games };
    }

    async fn get_collections(&self) {
        let collection_names = self.db.list_collection_names(None).await.unwrap();
        for collection_name in collection_names {
            println!("{}", collection_name);
        }
    }

    async fn insert_history(&self, doc: &GameHistory){
        let res: InsertOneResult;
        match self.game_history.insert_one(doc, None).await {
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
        let mut cursor = self.game_history.find(None, None).await.unwrap();
        while let Some(doc) = cursor.next().await {
            println!("{:?}", doc);
        }
    }

    pub async fn get_ongoing_game_by_user_id(&self, user_id: &String) -> Result<OngoingGame, ()> {
        let result = self.ongoing_games.find_one(
            doc! { "$or": [{"player_one": &user_id }, {"player_two": &user_id }]},
            None
        ).await;
        if result.is_ok() {
            let ongoing_game = result.unwrap().unwrap();
            return Ok(ongoing_game);
        }
        return Err(());
    }

    pub async fn get_all_ongoing_games_by_user_id(&self, user_id: &String) -> Result<Vec<OngoingGame>, ()> {
        let mut games: Vec<OngoingGame> = Vec::new();
        let mut cursor = self.ongoing_games.find(
            doc! { "$or": [{"player_one": &user_id }, {"player_two": &user_id }]},
            None
        ).await.unwrap();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    games.push(doc);
                },
                Err(e) => {
                    println!("Error: {}", e);
                    // Handle error
                }
            }
        }
        return Ok(games);
    }

    pub async fn get_ongoing_game(&self, _id: &String) -> Result<OngoingGame, ()> {
        let result = self.ongoing_games.find_one(
            doc! { "_id": _id},
            None
        ).await;
        if result.is_ok() {
            let ongoing_game = result.unwrap().unwrap();
            return Ok(ongoing_game);
        }
        return Err(());
    }

    pub async fn insert_onging_game(&self, doc: &OngoingGame) -> MongoResult<()> {
        match self.ongoing_games.insert_one(doc, None).await {
            Ok(result) => {
                Ok(())
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    async fn update_onging_game(&self, doc: &OngoingGame) {
        // This function assumes all validation is done by the caller.
        let filter = doc! { "_id": doc.get_id() };
        // assert state is different
        let doc_bson= bson::to_bson(doc).unwrap();
        let update_doc = match doc_bson {
            bson::Bson::Document(document) => document,
            _ => return (),
        };
        let res = self.ongoing_games.update_one(filter, update_doc, None).await.unwrap();
        println!("Updated documents: {}", res.modified_count);
    }

    // pub async fn find_histories_by_player(&self, player_id: Uuid) -> Result<GameHistory, ()>{
    //     let player = player_id.to_string();
    //     let filter = doc! {
    //         "$or": [
    //             { "player_one": &player },
    //             { "player_two": &player }
    //         ]
    //     };

        // match self.game_histories.find(filter, None).await {
        //     Ok(Some(document)) => {
        //         let game_history: GameHistory = document;
        //         Ok(game_history)
        //     },
        //     Ok(None) => Err(()),
        //     Err(_) => Err(())
        // }
    // }
}
