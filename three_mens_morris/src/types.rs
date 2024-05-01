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
