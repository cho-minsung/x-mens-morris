use serde::{Deserialize, Serialize};

// database collection games
// has a key of game and stores uuid as string
// has a column for player 1 and stores uuid as string
// has a column for player 2 and stores uuid as string
// has a column named "bot" and store uuid as string if that player is a bot
// has a column named "winner" and store uuid as string
// has a column named "moves" and store string moves delimited by space

#[derive(Serialize, Deserialize, Debug)]
pub struct GameHistory {
    pub _id: String,
    pub player_one: String,
    pub player_two: String,
    pub bot: Option<String>,
    pub winner: Option<String>,
    pub moves: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub turn: u8, // 1 for 1st player, 2 for 2nd player
    pub player_one_remaining: u8,
    pub player_two_remaining: u8,
    pub board: [[u8; 3]; 3], // (row x col x 3) matrix where 0 is empty, 1 is 1st player, and 2 is 2nd player
}

impl State{
    pub fn new() -> State {
        State {
            turn: 1,
            player_one_remaining: 3,
            player_two_remaining: 3,
            board: [[0; 3]; 3],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OngoingGame {
    pub _id: String,
    pub player_one: String,
    pub player_two: String,
    pub current_state: State,
}

impl OngoingGame {
    pub fn new() -> OngoingGame{
        OngoingGame{
            _id: String::new(),
            player_one: String::new(),
            player_two: String::new(),
            current_state: State::new(),
        }
    }
}


