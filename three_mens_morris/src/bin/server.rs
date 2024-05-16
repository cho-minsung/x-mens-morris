
use std::collections::HashMap;

use rand::Rng;
use uuid::Uuid;

use actix_web::{get, post, patch, error, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use three_mens_morris::stupid_bot::StupidBot;
use three_mens_morris::database::TmmDbClient;
use three_mens_morris::referee::Referee;
use three_mens_morris::types::{ GameHistory, OngoingGame, Move};

const MAX_SIZE: usize = 262_144; // max payload size is 256k



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize)]
pub struct NewGamePayload {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovePayload {
    user_id: String,
    move_code: String,
}

impl MovePayload {
    pub fn new(user_id: String, move_code: String) -> Result<Self, &'static str> {
        if move_code.len() < 2 || move_code.len() > 4 {
            return Err("move_code must be 2 to 4 characters long");
        }

        Ok(Self { user_id, move_code })
    }
}

#[patch("/play")]
async fn play(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    // expect to have user_id and move in (char, u8) in payload
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MovePayload>(&body)?;

    let user_id = &obj.user_id;
    println!("move_code: {}", &obj.move_code);
    // TODO: check if move is valid
    let new_move = Move::string_to_move(&obj.move_code).unwrap();

    println!("got user ID {}", user_id);
    println!("got move {}", new_move.print());
    
    // Retrieve ongoing game
    let client = TmmDbClient::new().await;
    let ongoing_game = client.get_ongoing_game_by_user_id(user_id).await.unwrap();

    // Check whether piece is new or not
    if new_move.new_col.is_none() {
        // move is new
        
    }

    // Return newly generated game_id and the opponent.
    let mut response = HashMap::new();
    // response.insert("game_id", new_game._id);
    response.insert("player one", ongoing_game.player_one);
    response.insert("player two", ongoing_game.player_two);
    // response.insert("opponent_id", opponent.to_string());
    // response.insert("turn", new_state.turn.to_string());
    response.insert("player_one_remaining", ongoing_game.player_one_remaining.to_string());
    response.insert("player_two_remaining", ongoing_game.player_two_remaining.to_string());
    // response.insert("board", new_state.flatten_board());
    Ok(HttpResponse::Ok().json(response))
}

#[get("/get/{game_id}")]
async fn get_game(path: web::Path<String,>) -> Result<HttpResponse, Error> {
    let game_id = path.into_inner();
    println!("game ID: {}", game_id);
    
    // Retrieve ongoing game
    let client = TmmDbClient::new().await;
    let ongoing_game = client.get_ongoing_game(&game_id).await.unwrap();

    // // Return newly generated game_id and the opponent.
    let mut response = HashMap::new();
    response.insert("game_id", ongoing_game._id.to_string());
    response.insert("player one", ongoing_game.player_one.to_string());
    response.insert("player two", ongoing_game.player_two.to_string());
    // response.insert("opponent_id", opponent.to_string());
    response.insert("turn", ongoing_game.whose_turn.to_string());
    response.insert("player_one_remaining", ongoing_game.player_one_remaining.to_string());
    response.insert("player_two_remaining", ongoing_game.player_two_remaining.to_string());
    response.insert("board", ongoing_game.flatten_board());
    // response.insert("moves", ongoing_game.moves.iter().map(|i| i.print()).collect::<Vec<String>>());
    Ok(HttpResponse::Ok().json(response))
}

#[get("/games/{player_id}")]
async fn get_games_by_player_id(path: web::Path<String,>) -> Result<HttpResponse, Error> {
    // get all game ID this player is currently playing
    let player_id = path.into_inner();
    println!("player ID: {}", player_id);
    
    // Retrieve ongoing game
    let client = TmmDbClient::new().await;
    let ongoing_games = client.get_all_ongoing_games_by_user_id(&player_id).await.unwrap();

    let mut response = HashMap::new();
    for (i, game) in ongoing_games.iter().enumerate() {
        let key = format!("game_{}", i);
        response.insert(key, game._id.to_string());
    }
    
    Ok(HttpResponse::Ok().json(response))
}

#[post("/new")]
async fn start_new_game(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let client: TmmDbClient = TmmDbClient::new().await;

    // payload is a stream of Bytes objects
    // expect to have user_id in payload
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<NewGamePayload>(&body)?;
    // TODO: check user id valid

    // initialize new OngoingGame
    let mut new_game: OngoingGame = OngoingGame::new();
    // generate a uuid for a game.
    new_game._id = Uuid::new_v4().to_string();
    // as proof of concept, let user pass down their uuid in the payload
    let user_id = &obj.user_id;

    // create stupid bot opponent
    let bot = StupidBot::new();
    let opponent = bot.get_id();
    
    // flip a coin:
    // head -> user=player one
    // tail -> bot=player one
    let mut rng = rand::thread_rng();
    let random_bool: bool = rng.gen();
    match random_bool {
        true => { // user goes first
            new_game.player_one = user_id.clone();
            new_game.player_two = opponent.clone();
        },
        false => {
            new_game.player_one = opponent.clone();
            new_game.player_two = user_id.clone();
            
        }
    }

    new_game.whose_turn = new_game.player_one.clone();
    // let bot play first
    let _ = bot.place_random_new_piece(&mut new_game);

    let player_one_remaining = new_game.player_one_remaining.to_string();
    let player_two_remaining = new_game.player_two_remaining.to_string();
    let flattened_board = new_game.flatten_board();

    // create the ongoing game to database
    let result = client.insert_onging_game(&new_game).await;
    match result {
        Ok(()) => {},
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(
                format!("Failed to insert game: {}", e)));
        },
    }
    
    // Response to return
    let mut response = HashMap::new();
    response.insert("game_id", new_game.get_id());
    response.insert("player_one", &new_game.player_one);
    response.insert("player_two", &new_game.player_two);
    response.insert("turn", &new_game.whose_turn);
    response.insert("player_one_remaining", &player_one_remaining);
    response.insert("player_two_remaining", &player_two_remaining);
    response.insert("board", &flattened_board);

    Ok(HttpResponse::Ok().json(response))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Insert a new document into the collection.
    // let doc = GameHistory {
    //     _id: String::from("036d2541-b81f-40f9-baf6-8cd8a1d589c9"),
    //     player_one: String::from("c152e455-5609-4031-afeb-fa63b938de5f"),
    //     player_two: String::from("e178c427-0c16-446d-98c0-51a3dff6d7e4"),
    //     winner: Some(String::from("036d2541-b81f-40f9-baf6-8cd8a1d589c9")),
    //     moves: String::from("a1 b1 a2 b2 a3"),
    // };

    // let find_doc = client.find_history_by_player(Uuid::parse_str("c152e455-5609-4031-afeb-fa63b938de5f").unwrap()).await.unwrap();
    // println!("{:?}", find_doc);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(play)
            .service(echo)
            .service(get_game)
            .service(start_new_game)
            .service(get_games_by_player_id)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
