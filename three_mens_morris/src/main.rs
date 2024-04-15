use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
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
    NotPlayedAllPieces { player: char },
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
            GameError::NotPlayedAllPieces { player } => {
                write!(f, "Player {} must play remaining pieces to start moving existing pieces!", player)
            }
            GameError::CustomError { message } => {
                write!(f, "Custom error: {}", message)
            }
        }
    }
}

impl std::error::Error for GameError {}

pub struct Move {
    // Move is a human-readible symantic move record
    col: char,
    row: u8,
    new_col: Option<char>,
    new_row: Option<u8>,
}

impl Move {
    pub fn is_new_move(&self) -> bool {
        return self.new_row.is_none()
    }
    
    pub fn new_as_coord(&self) -> (usize, usize) {
        match self.col {
            'A' | 'a' => {
                return (self.row as usize - 1, 0)
            },
            'B' | 'b' => {
                return (self.row as usize - 1, 1)
            },
            'C' | 'c' => {
                return (self.row as usize - 1, 2)
            },
            _ => return (3, 3)
        }
    }

    pub fn move_as_coord(&self) -> (usize, usize) {
        if self.is_new_move() { return (3, 3) };
        match self.new_col.unwrap() {
            'A' | 'a' => {
                return (self.new_row.unwrap() as usize - 1, 0)
            },
            'B' | 'b' => {
                return (self.new_row.unwrap() as usize - 1, 1)
            },
            'C' | 'c' => {
                return (self.new_row.unwrap() as usize - 1, 2)
            },
            _ => return (3, 3)
        }
    }

}

#[derive(Clone)]
pub struct State {
    turn: u8, // 1 for 1st player, 2 for 2nd player
    player_one_remaining: u8,
    player_two_remaining: u8,
    board: [[u8; 3]; 3], // (row x col x 3) matrix where 0 is empty, 1 is 1st player, and 2 is 2nd player
}

impl State {
    pub fn new() -> State {
        State {
            turn: 1,
            player_one_remaining: 3,
            player_two_remaining: 3,
            board: [[0; 3]; 3],
        }
    }

    pub fn get_state(&self) -> String {
        // Print current state in NN input format
        // first print current turn (1 or 2)
        // second, print number of piece left to play for players
        // third, print 3x3 from a1, a2, ... c2, c3 (0 if empty, 1 or 2)
        let mut state = vec![
            self.turn.to_string(),
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

pub struct Game {
    // column, and row
    moves: Vec<Move>,
    state_history: Vec<State>,
    current_state: State,
    winner: char,
    player_mode: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            moves: Vec::new(),
            state_history: Vec::new(),
            current_state: State::new(),
            winner: ' ',
            player_mode: 2,
        }
    }

    pub fn write_history(&self) {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backward?");
        let timestamp = time.as_secs();
        let path = format!("{}.csv", timestamp);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path)
            .expect("Unable to open file.");
        for state in &self.state_history {
            writeln!(file, "{}", state.get_state()).expect("Corrupted history.");
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
            println!("Current state:");
            println!("{}", self.current_state.get_state());
            let mut input = String::new();
            println!("Please enter some input: ");
            std::io::stdin().read_line(&mut input).unwrap();
            // print!("\x1B[2J\x1B[1;1H");
            let words: Vec<&str> = input.split_whitespace().collect();
            let new_move = match self.validate_input(words) {
                Ok(_move) => _move,
                Err(e) => {
                    println!("{}", e);
                    println!("Let's try this again.");
                    continue;
                }
            };

            match self.validate_move(&new_move) {
                Ok(()) => {
                    self.register_move(new_move);
                },
                Err(e) => {
                    println!("{}", e);
                    println!("Let's try this again.");
                    continue;
                }
            }

            match self.validate_win() {
                Ok(win) => {
                    if win {
                        match self.current_state.turn {
                            1 => {
                                self.winner = 'o'
                            },
                            2 => {
                                self.winner = 'x'
                            },
                            _ => {
                                self.winner = ' '
                            }
                        }
                        println!("Player {} is the winner!", self.winner);
                        self.print_current_board();
                    }
                },
                Err(_) => { println!("Unknown error!") }
            }

            
        }
        println!("Game over!");
        self.print_move_history();
    }

    pub fn print_move_history(&self) {
        println!("Game history:");
        for (i, current_move) in self.moves.iter().enumerate() {
            let mut player;
            match i % 2 {
                0 => {
                    player = 'o';
                },
                1 => {
                    player = 'x';
                },
                _ => {
                    // TODO: handle error
                    player = ' ';
                }
            }

            if current_move.new_row.is_none() {
                println!("{}: {}{} ", player, current_move.col, current_move.row);
            }
            else {
                println!("{}: {}{}->{}{}", player, current_move.col, current_move.row, current_move.new_col.unwrap(), current_move.new_row.unwrap());
            }
        }
    }

    pub fn print_current_board(&self) {
        // Print current board on cli
        println!("  a   b   c");
        for (i, row) in self.current_state.board.iter().enumerate() {
            print!("{} ", i+1);
            for (j, &cell) in row.iter().enumerate() {
                let char_rep = match cell {
                    1 => 'o',
                    2 => 'x',
                    _ => ' '
                };
                if j == row.len() - 1 {
                    print!("{}\n", char_rep);
                }
                else {
                    print!("{} {} ", char_rep, "-".red());
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
        // before updating current state, save it to history
        self.state_history.push(self.current_state.clone());
        // apply on current board first
        let ( row_coord, col_coord ) = _move.new_as_coord();
        // new piece
        if _move.is_new_move() {
            self.current_state.board[row_coord][col_coord] = self.current_state.turn;
        }
        // moving piece
        else {
            self.current_state.board[row_coord][col_coord] = 0;
            let ( new_row_coord, new_col_coord ) = _move.move_as_coord();
            self.current_state.board[new_row_coord][new_col_coord] = self.current_state.turn;
        }
        match self.current_state.turn {
            1 => {
                self.current_state.turn = 2;
                if self.current_state.player_one_remaining > 0 { self.current_state.player_one_remaining -= 1 };
            },
            2 => {
                self.current_state.turn = 1;
                if self.current_state.player_two_remaining > 0 { self.current_state.player_two_remaining -= 1 };
            },
            _ => (),
        }

        println!("Move has been registered:");
        // record Move to move history
        self.moves.push(_move);
        self.print_current_board();
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
        let board = &self.current_state.board;
        for i in 0..3 {
            if (board[i][0] != 0 && board[i][0] == board[i][1] && board[i][0] == board[i][2])
                || (board[0][i] != 0 && board[0][i] == board[1][i] && board[0][i] == board[2][i]) {
                self.write_history();
                return Ok(true)
            }
        }
    
        // Check diagonals
        if (board[0][0] != 0 && board[0][0] == board[1][1] && board[0][0] == board[2][2])
            || (board[0][2] != 0 && board[0][2] == board[1][1] && board[0][2] == board[2][0]) {
            self.write_history();
            return Ok(true)
        }

        return Ok(false)
    }

    pub fn validate_move(&mut self, _move: &Move) -> Result<(), GameError> {
        // validate_move takes Move and outputs error if invalid.
        let (row_coord, col_coord) = _move.new_as_coord();
        let current_player = match self.current_state.turn {
            1 => 'o',
            2 => 'x',
            _ => ' ',
        };

        // placing new piece
        if _move.is_new_move() {
            // check if place already has a piece
            if self.current_state.board[row_coord][col_coord] != 0 {
                return Err(GameError::PlaceOccupied { row: row_coord, col: col_coord });
            }
            match self.current_state.turn {
                1 => {
                    if self.current_state.player_one_remaining == 0 {
                        return Err(GameError::MaxPiecePlayed { player: current_player });
                    }
                }
                2 => {
                    if self.current_state.player_two_remaining == 0 {
                        return Err(GameError::MaxPiecePlayed { player: current_player });
                    }
                },
                _ => {
                    return Err(GameError::CustomError { message: "unknown turn." })
                }
            }
            return Ok(())
        }

        // moving existing piece
        // validate if piece exists to move
        if self.current_state.board[row_coord][col_coord] == 0 {
            return Err(GameError::NoPieceToMove { row: row_coord, col: col_coord })
        }
        // validate if all remaining pieces are played.
        if self.current_state.player_one_remaining > 0 || self.current_state.player_two_remaining > 0 {
            return Err(GameError::NotPlayedAllPieces { player: current_player })
        }
        // validate if initial position belongs to player
        if self.current_state.turn != self.current_state.board[row_coord][col_coord] {
            return Err(GameError::IncorrectOwnership { player: current_player, row: row_coord, col: col_coord });
        }
        // try unwrapping new row and new columns
        // convert new col and row to cartesian coordinates
        let (new_row_coord, new_col_coord) = _move.move_as_coord();
        // validate if new position is not occupied
        if self.current_state.board[new_row_coord][new_col_coord] != 0 {
            return Err(GameError::PlaceOccupied { row: new_row_coord, col: new_col_coord })
        }
        // player can only move to connected grid.
        if !self.is_valid_move(row_coord, col_coord, new_row_coord, new_col_coord) {
            return Err(GameError::InvalidMove {})
        }
        
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
        // validate_input takes input vector and outputs Move
        // column is from a to c, row is from 1 to 3
        if input.len() > 2{
            return Err("Incorrect amount of arguments.")
        }
        
        // check col and row from second argument
        let row;
        let col;
        match self.convert_str_to_row_col(input[0]) {
            Ok((parsed_col, parsed_row)) => {
                row = parsed_row;
                col = parsed_col;
            },
            Err(err) => return Err(err)
        }

        // New move.
        if input.len() == 1 {
            return Ok(
                Move {
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
        match self.convert_str_to_row_col(input[1]) {
            Ok((parsed_col, parsed_row)) => {
                new_row = parsed_row;
                new_col = parsed_col;
            },
            Err(err) => return Err(err)
        }

        Ok(Move {
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