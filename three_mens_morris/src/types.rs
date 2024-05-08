use serde::{Deserialize, Serialize};

// database collection games
// has a key of game and stores uuid as string
// has a column for player 1 and stores uuid as string
// has a column for player 2 and stores uuid as string
// has a column named "bot" and store uuid as string if that player is a bot
// has a column named "winner" and store uuid as string
// has a column named "moves" and store string moves delimited by space

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Move {
    // Move is a human-readible symantic move record
    pub col: char,
    pub row: u8,
    pub new_col: Option<char>,
    pub new_row: Option<u8>,
}

impl Move {
    pub fn new() -> Move {
        Move {
            col: 'a',
            row: 0,
            new_col: None,
            new_row: None
        }
    }

    pub fn convert_to_new_move(row: &usize, col: &usize) -> Result<Self, ()> {
        // acceptable rows = 0 to 2
        if *row > 2 {
            return Err(());
        }
        let row = *row as u8 + 1;
        let col = match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => return Err(()),
        };

        Ok(
            Self {
                col: col,
                row: row,
                new_col: None,
                new_row: None,
        })
    }

    pub fn convert_to_move(row: &usize, col: &usize, new_row: &usize, new_col: &usize) -> Result<Self, ()> {
        // acceptable rows = 0 to 2
        if *row > 2 {
            return Err(());
        }

        let col = match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => return Err(()),
        };
        let row = *row as u8 + 1;

        let new_col = match new_col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => ' ',
        };
        let new_row = *new_row as u8 + 1;

        Ok(Self {
            col: col,
            row: row,
            new_col: Some(new_col),
            new_row: Some(new_row),
        })
    }

    pub fn is_new_move(&self) -> bool {
        return self.new_row.is_none();
    }

    pub fn new_as_coord(&self) -> (usize, usize) {
        match self.col {
            'A' | 'a' => return (self.row as usize - 1, 0),
            'B' | 'b' => return (self.row as usize - 1, 1),
            'C' | 'c' => return (self.row as usize - 1, 2),
            _ => return (3, 3),
        }
    }

    pub fn move_as_coord(&self) -> (usize, usize) {
        if self.is_new_move() {
            return (3, 3);
        };
        match self.new_col.unwrap() {
            'A' | 'a' => return (self.new_row.unwrap() as usize - 1, 0),
            'B' | 'b' => return (self.new_row.unwrap() as usize - 1, 1),
            'C' | 'c' => return (self.new_row.unwrap() as usize - 1, 2),
            _ => return (3, 3),
        }
    }

    pub fn print(&self) -> String {
        let mut output = String::new();
        // print order e.g. a1
        output.push(self.col);
        output.push_str(&self.row.to_string());
        
        if self.new_col.is_some() {
            output.push_str(&String::from("->"));
            output.push(self.new_col.unwrap());
            output.push_str(&self.new_row.unwrap().to_string());
        }
        output
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameHistory {
    pub _id: String,
    pub player_one: String,
    pub player_two: String,
    pub winner: String,
    pub moves: Vec<(char, char)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OngoingGame {
    pub _id: String,
    pub player_one: String,
    pub player_two: String,
    pub whose_turn: String,
    // player one is 1 and player two is 2 for peice
    pub player_one_remaining: u8,
    pub player_two_remaining: u8,
    pub board: [[u8; 3]; 3], // (row x col x 3) matrix where 0 is empty, 1 is 1st player, and 2 is 2nd player
    pub moves: Vec<Move>,
}

impl OngoingGame {
    pub fn new() -> OngoingGame{
        OngoingGame{
            _id: String::new(),
            player_one: String::new(),
            player_two: String::new(),
            whose_turn: String::new(),
            player_one_remaining: 3,
            player_two_remaining: 3,
            board: [[0; 3]; 3],
            moves: Vec::new(),
        }
    }

    pub fn flatten_board(&self) -> String {
        let mut board_flattened = String::new();
        for row in self.board {
            for content in row {
                board_flattened.push_str(&content.to_string());
            }
        }
        return board_flattened;
    }

    pub fn get_id(&self) -> &String {
        return &self._id;
    }

    pub fn get_player_one(&self) -> &String {
        return &self.player_one;
    }

    pub fn update_turn(&mut self) {
        match self.whose_turn == self.player_one {
            true => self.whose_turn = self.player_two.clone(),
            false => self.whose_turn = self.player_one.clone(),
        }
    }

    pub fn get_state(&self) -> String {
        // Print current state in NN input format
        // first print current turn (1 or 2)
        // second, print number of piece left to play for players
        // third, print 3x3 from a1, a2, ... c2, c3 (0 if empty, 1 or 2)
        let mut state = vec![
            self.whose_turn.to_string(),
            self.player_one_remaining.to_string(),
            self.player_two_remaining.to_string(),
        ];
        for row in 0..3 {
            for col in 0..3 {
                state.push(self.board[row][col].to_string());
            }
        }
        state.join(",")
    }
}


