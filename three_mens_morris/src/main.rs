use std::collections::HashMap;
use std::fmt;
use colored::*;

#[derive(Debug)]
pub enum GameError {
    // move related errors
    DuplicatePlay { player: char },
    PlaceOccupied { row: usize, col: usize },
    MaxPiecePlayed { player: char },
    NoPieceToMove { row: usize, col: usize },
    IncorrectOwnership { player: char, row: usize, col: usize },
    InvalidMove {},
    NotPlayedAllPieces { player: char, remaining: u8 },
    CustomError { message: &'static str },

    // input related errors
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::DuplicatePlay { player } => {
                write!(f, "Player {} cannot play twice in a row.", player)
            }
            GameError::PlaceOccupied { row, col } => {
                write!(f, "({}, {}) already occupied!", row, col)
            }
            GameError::MaxPiecePlayed { player } => {
                write!(f, "Player {} already placed all 3 pieces!", player)
            }
            GameError::NoPieceToMove { row, col } => {
                write!(f, "({}, {}) has no piece to move!", row, col)
            }
            GameError::IncorrectOwnership { player, row, col } => {
                write!(f, "Piece on ({}, {}) does not belong to player {}!", row, col, player)
            }
            GameError::InvalidMove {} => {
                write!(f, "The move is not valid!")
            }
            GameError::NotPlayedAllPieces { player, remaining } => {
                write!(f, "Player {} must play {} remaining pieces to start moving existing pieces!", player, remaining)
            }
            GameError::CustomError { message } => {
                write!(f, "Custom error: {}", message)
            }
        }
    }
}

impl std::error::Error for GameError {}

pub struct Move {
    player: char,
    col: char,
    row: u8,
    new_col: Option<char>,
    new_row: Option<u8>,
}

pub struct Game {
    // column, and row
    moves: Vec<Move>,
    current_board: [[char; 3]; 3], // current_board[row][col],
    player_piece_count: HashMap<char, u8>,
    winner: char,
    player_mode: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            moves: Vec::new(),
            current_board: [[' '; 3]; 3],
            player_piece_count: HashMap::new(),
            winner: ' ',
            player_mode: 2,
        }
    }

    pub fn get_player_mode(&mut self) -> bool {
        let mut input = String::new();
        println!("Please enter 1 for single player mode or 2 for multiplayer mode:");
        std::io::stdin().read_line(&mut input).unwrap();
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() != 1 {
            println!("Limit your input to one argument!");
            return false
        }
        let mode: u8 = match words[0].parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Error parsing player mode.");
                return false
            },
        };
        match mode {
            1 => {
                self.player_mode = mode;
                println!("Single player mode selected.");
                return true
            },
            2 => {
                self.player_mode = mode;
                println!("Multi player mode selected.");
                return true
            },
            _ => {
                println!("Unsupported player mode.");
                return false
            }
        }
    }

    pub fn start(&mut self) {
        println!("Welcome to Three Men's Morris!");
        let mut player_mode_valid = false;
        while !player_mode_valid {
            player_mode_valid = self.get_player_mode();
        }

        self.print_current_board();
       
        while self.winner == ' ' {
            let mut input = String::new();
            println!("Please enter some input: ");
            std::io::stdin().read_line(&mut input).unwrap();
            // print!("\x1B[2J\x1B[1;1H");
            let words: Vec<&str> = input.split_whitespace().collect();
            let new_move: Move;
            match self.validate_input(words) {
                Ok(_move) => {
                    new_move = _move;
                },
                Err(e) => {
                    println!("{}", e);
                    println!("Let's try this again.");
                    continue;
                }
            };
            match self.validate_move(new_move) {
                Ok(()) => {
                },
                Err(e) => {
                    println!("{}", e);
                    println!("Let's try this again.");
                    continue;
                }
            }
        }
        println!("Game over!");
        self.print_move_history();
    }

    pub fn print_move_history(&self) {
        println!("Game history:");
        for (i, move_history) in self.moves.iter().enumerate() {
            if move_history.new_row.is_none() {
                println!("{}: Player {} placed {}{}", i, move_history.player, move_history.col, move_history.row);
            }
            else {
                println!("{}: Player {} moved from {}{} to {}{}",
                i, move_history.player, move_history.col, move_history.row, move_history.new_col.unwrap(), move_history.new_row.unwrap());
            }
        }
    }

    pub fn print_current_board(&self) {
        // Print current board on cli
        println!("  a   b   c");
        for (i, row) in self.current_board.iter().enumerate() {
            print!("{} ", i+1);
            for (j, &cell) in row.iter().enumerate() {
                if j == row.len() - 1 {
                    print!("{}\n", cell);
                }
                else {
                    print!("{} {} ", cell, "-".red());
                }
            }
            if i == 0 {
                println!("  {}", "| \\ | / |".red());
            }
            else if i == 1 {
                println!("  {}", "| / | \\ |".red());
            }
            else {
                println!();
            }
        }
    }

    pub fn register_move(&mut self, _move: Move) {
        println!("Move has been registered:");
        self.moves.push(_move);
        self.print_current_board();
        match self.validate_win() {
            Ok(win) => {
                if win {
                    self.winner = self.moves.last().unwrap().player;
                    println!("Player {} is the winner!", self.winner);
                    self.print_current_board();
                }
            },
            Err(_) => { println!("Unknown error!") }
        }
    }

    pub fn is_valid_move(&self, old_row: usize, old_col: usize, new_row: usize, new_col: usize) -> bool {
        // possible moves:
        // +/- column
        // +/- row
        // col+1 and row+1
        // col-1 and row-1
        // It calculates the absolute difference between the old and new row and column, and checks if these differences are at most 1.
        let exceptions = vec![
            ((0, 1), (1, 0)),
            ((0, 1), (1, 2)),
            ((1, 0), (2, 1)),
            ((2, 1), (1, 2)),
        ];
        
        let current_move = ((old_row, old_col), (new_row, new_col));
        let reverse_move = ((new_row, new_col), (old_row, old_col));

        if exceptions.contains(&current_move) || exceptions.contains(&reverse_move) {
            return false
        }

        let row_diff = (old_row as i32 - new_row as i32).abs();
        let col_diff = (old_col as i32 - new_col as i32).abs();
        
        row_diff <= 1 && col_diff <= 1
    }

    pub fn validate_win(&self) -> Result<bool, &'static str> {
        let board = &self.current_board;
        for i in 0..3 {
            if (board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][0] == board[i][2])
                || (board[0][i] != ' ' && board[0][i] == board[1][i] && board[0][i] == board[2][i]) {
                return Ok(true)
            }
        }
    
        // Check diagonals
        if (board[0][0] != ' ' && board[0][0] == board[1][1] && board[0][0] == board[2][2])
            || (board[0][2] != ' ' && board[0][2] == board[1][1] && board[0][2] == board[2][0]) {
            return Ok(true)
        }

        return Ok(false)
    }

    pub fn convert_char_to_coordinate(&self, col: char) -> Result<usize, &'static str> {
        match col {
            'A' | 'a' => {
                return Ok(0)
            },
            'B' | 'b' => {
                return Ok(1)
            },
            'C' | 'c' => {
                return Ok(2)
            },
            _ => return Err("Error converting column to coordinate.")
        }
    }

    pub fn validate_move(&mut self, _move: Move) -> Result<(), GameError> {
        // start of the game
        // if there are no moves, then register first player
        // register the players
        if self.player_piece_count.is_empty() {
            self.player_piece_count.insert(_move.player, 0);
            println!("Player one has been registered as {}", _move.player);
        }
        else if self.player_piece_count.len() == 1 {
            self.player_piece_count.insert(_move.player, 0);
            println!("Player two has been registered as {}", _move.player);
        }

        if !self.moves.is_empty() {
            // Reject same player to play twice.
            let last_player = self.moves.last().map(|m| m.player).unwrap();
            if last_player == _move.player {
                return Err(GameError::DuplicatePlay { player: _move.player });
            }
        }

        // convert col and row to cartesian coordinates
        let row_coord = _move.row as usize - 1;
        let col_coord = match self.convert_char_to_coordinate(_move.col) {
            Ok(new_col) => {new_col},
            Err(err) => return Err(GameError::CustomError{message: err})
        };

        // placing new piece
        if _move.new_row.is_none() && _move.new_col.is_none() {
            // check if place already has a piece
            if self.current_board[row_coord][col_coord] != ' ' {
                return Err(GameError::PlaceOccupied { row: row_coord, col: col_coord });
            }
            let piece_count = *self.player_piece_count.get(&_move.player).unwrap();
            if piece_count >= 3 {
                return Err(GameError::MaxPiecePlayed { player: _move.player });
            }
            self.current_board[row_coord][col_coord] = _move.player;
            *self.player_piece_count.entry(_move.player).or_insert(0) += 1;
            self.register_move(_move);
            return Ok(())
        }

        // moving existing piece
        let piece_count = *self.player_piece_count.get(&_move.player).unwrap();
        if piece_count != 3 {
            return Err(GameError::NotPlayedAllPieces { player: _move.player, remaining: 3 - piece_count })
        }
        // validate if initial position belongs to player
        if _move.player != self.current_board[row_coord][col_coord] {
            return Err(GameError::IncorrectOwnership { player: _move.player, row: row_coord, col: col_coord });
        }
        // validate if piece exists to move
        if self.current_board[row_coord][col_coord] == ' ' {
            return Err(GameError::NoPieceToMove { row: row_coord, col: col_coord })
        }

        // try unwrapping new row and new columns
        // convert new col and row to cartesian coordinates
        let new_col_coord = match self.convert_char_to_coordinate(_move.new_col.unwrap()) {
            Ok(new_col) => {new_col},
            Err(err) => return Err(GameError::CustomError{message: err})
        };
        let new_row_coord = _move.new_row.unwrap() as usize - 1;

        // validate if new position is not occupied
        if self.current_board[new_row_coord][new_col_coord] != ' ' {
            return Err(GameError::PlaceOccupied { row: new_row_coord, col: new_col_coord })
        }

        // player can only move to connected grid.
        if !self.is_valid_move(row_coord, col_coord, new_row_coord, new_col_coord) {
            return Err(GameError::InvalidMove {})
        }
        
        self.current_board[row_coord][col_coord] = ' ';
        self.current_board[new_row_coord][new_col_coord] = _move.player;
        self.register_move(_move);
        return Ok(())
    }

    pub fn convert_str_to_row_col(&self, move_entry: &str) -> Result<(char, u8), &'static str> {
        if move_entry.len() != 2 {
            return Err("Invalid move!");
        }

        let chars: Vec<char> = move_entry.chars().collect();
        let row = match chars[1].to_digit(10) {
            Some(value) => value as u8,
            None => return Err("Invalid column!"),
        };
        let col = chars[0];

        // Validate row and column entries
        if (row < 1) && (row > 3) {
            return Err("Invalid row argument!")
        };
        let valid_cols = vec!['a', 'b', 'c', 'A', 'B', 'C'];
        if !valid_cols.contains(&col) {
            return Err("Invalid column argument!")
        };

        Ok((col, row))
    }

    pub fn validate_input(&mut self, input: Vec<&str>) -> Result<Move, &'static str> {
        // column is from a to c, row is from 1 to 3
        if input.len() != 2 && input.len() != 3{
            return Err("Incorrect amount of arguments.")
        }
        // check user value if both player characters are registred
        let user: char = match input[0].chars().next() {
            Some(character) => {
                let invalid_chars = vec!['_', ' ', '\\', '/', '|', 'a', 'b', 'c', '1', '2', '3', 'A', 'B', 'C'];
                if invalid_chars.contains(&character) {
                    return Err("Invalid user argument!");
                }
                character
            },
            None => return Err("Invalid user argument!"),
        };
        if self.player_piece_count.len() >= 2 && !self.player_piece_count.contains_key(&user) {
            return Err("Two players are already registered.")
        }
        
        // check col and row from second argument
        let row;
        let col;
        match self.convert_str_to_row_col(input[1]) {
            Ok((parsed_col, parsed_row)) => {
                row = parsed_row;
                col = parsed_col;
            },
            Err(err) => return Err(err)
        }

        // New move.
        if input.len() == 2 {
            return Ok(
                Move {
                    player: user,
                    col: col,
                    row: row,
                    new_col: None,
                    new_row: None,
                }
            )
        };

        // check new col and new row
        let new_row;
        let new_col;
        // check col and row from second argument
        match self.convert_str_to_row_col(input[2]) {
            Ok((parsed_col, parsed_row)) => {
                new_row = parsed_row;
                new_col = parsed_col;
            },
            Err(err) => return Err(err)
        }

        Ok(Move {
                player: user,
                col: col,
                row: row,
                new_col: Some(new_col),
                new_row: Some(new_row),
            }
        )
    }
}

fn main() {
    let mut game = Game::new();
    game.start();
}