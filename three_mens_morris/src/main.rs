use colored::*;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

mod referee;
use crate::referee::Referee;

mod state;
use crate::state::State;

mod move_def;
use crate::move_def::Move;

mod stupid_bot;
use crate::stupid_bot::StupidBot;

#[derive(Debug)]
pub enum GameError {
    // move related errors
    DuplicatePlay {
        player: char,
    },
    PlaceOccupied {
        row: usize,
        col: usize,
    },
    MaxPiecePlayed {
        player: char,
    },
    NoPieceToMove {
        row: usize,
        col: usize,
    },
    IncorrectOwnership {
        player: char,
        row: usize,
        col: usize,
    },
    InvalidMove {},
    NotPlayedAllPieces {
        player: char,
    },
    CustomError {
        message: &'static str,
    },
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
                write!(
                    f,
                    "Piece on ({}, {}) does not belong to player {}!",
                    row, col, player
                )
            }
            GameError::InvalidMove {} => {
                write!(f, "The move is not valid!")
            }
            GameError::NotPlayedAllPieces { player } => {
                write!(
                    f,
                    "Player {} must play remaining pieces to start moving existing pieces!",
                    player
                )
            }
            GameError::CustomError { message } => {
                write!(f, "Custom error: {}", message)
            }
        }
    }
}

impl std::error::Error for GameError {}

pub struct Game {
    // column, and row
    moves: Vec<Move>,
    state_history: Vec<State>,
    current_state: State,
    winner: char,
    player_mode: u8,
    bot: StupidBot,
}

impl Game {
    pub fn new() -> Game {
        Game {
            moves: Vec::new(),
            state_history: Vec::new(),
            current_state: State::new(),
            winner: ' ',
            player_mode: 2,
            bot: StupidBot::new(),
        }
    }

    pub fn write_history(&self) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward?");
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

    pub fn get_player_mode(&mut self) -> Result<(), ()> {
        let mut input = String::new();
        println!("Please enter 1 for single player mode or 2 for multiplayer mode:");
        std::io::stdin().read_line(&mut input).unwrap();
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() != 1 {
            println!("Limit your input to one argument!");
            return Err(());
        }
        let mode: u8 = match words[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing player mode.");
                return Err(());
            }
        };
        match mode {
            1 => {
                self.player_mode = mode;
                println!("Single player mode selected.");
                println!("Fight against stupid bot.");
                // 1: player goes first, 2: bot goes first
                let mut rng = rand::thread_rng();
                let number: u8 = rng.gen_range(1..=2);
                match number {
                    1 => {
                        self.bot.set_player(1);
                        println!("Bot is set to player one.");
                    }
                    2 => {
                        self.bot.set_player(2);
                        println!("Bot is set to player two.");
                    }
                    _ => {
                        return Err(());
                    }
                }
                return Ok(());
            }
            2 => {
                self.player_mode = mode;
                println!("Multi player mode selected.");
                return Ok(());
            }
            _ => {
                println!("Unsupported player mode.");
                return Ok(());
            }
        }
    }

    fn multi_player(&mut self) -> u8 {
        // loop until there is a winner
        loop {
            // get input
            let new_move = match self.get_user_input() {
                Ok(ok_move) => ok_move,
                Err(()) => return 0,
            };
            // register input and move on if move is valid
            match self.validate_move(&new_move) {
                Ok(()) => {
                    self.register_move(&new_move);
                    let winner = self.check_win();
                    if winner != 0 {
                        return winner;
                    };
                    self.update_next_state();
                }
                Err(e) => {
                    println!("{}", e);
                    println!("Let's try this again.");
                }
            }
        }
    }

    fn single_player(&mut self) -> u8 {
        // loop until there is a winner
        loop {
            let mut new_move = Move::new();
            // let bot check if it is his turn first
            if self.bot.get_player() == self.current_state.turn {
                // random move is already validated
                match self.bot.make_random_move(&self.current_state) {
                    Ok(bot_move) => {
                        new_move = bot_move;
                        println!("bot move {} {} {:?} {:?}", new_move.row, new_move.col, new_move.new_row, new_move.new_col);
                        if new_move.is_new_move() {
                            println!("Bot has played {}{}", new_move.col, new_move.row);
                        } else {
                            println!(
                                "Bot has played {}{} -> {:?}{:?}",
                                new_move.col, new_move.row, new_move.new_col, new_move.new_row
                            );
                        }
                    }
                    Err(()) => {
                        println!("Bot had an error making a move.");
                    }
                }
            }
            // player register new move
            else {
                loop {
                    new_move = match self.get_user_input() {
                        Ok(ok_move) => ok_move,
                        Err(()) => return 0,
                    };
                    // register input and move on if move is valid
                    match self.validate_move(&new_move) {
                        Ok(()) => {break}
                        Err(e) => {
                            println!("{}", e);
                            println!("Error validating the move.");
                            continue
                        }
                    }
                }
            }

            self.register_move(&new_move);
            let winner = self.check_win();
            if winner != 0 {
                return winner;
            };
            self.update_next_state();
        }
    }

    fn get_user_input(&self) -> Result<Move, ()> {
        loop {
            println!("Please enter some input: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            // print!("\x1B[2J\x1B[1;1H");
            let words: Vec<&str> = input.split_whitespace().collect();
            match self.validate_input(words) {
                Ok(new_move) => return Ok(new_move),
                Err(e) => {
                    println!("{}", e);
                    println!("Let's try this again.");
                }
            };
        }
    }

    pub fn start(&mut self) {
        println!("Welcome to Three Men's Morris!");
        loop {
            match self.get_player_mode() {
                Ok(()) => break,
                Err(()) => {
                    println!("Wrong player mode!");
                    return;
                }
            }
        }

        self.print_current_board();

        let mut winner = 0;
        println!("Current state:");
        println!("{}", self.current_state.get_state());
        match self.player_mode {
            1 => {
                winner = self.single_player();
            }
            2 => {
                winner = self.multi_player();
            }
            _ => (),
        }
        println!("Player {} is the winner!", winner);
        self.print_current_board();
        self.print_move_history();
    }

    pub fn print_move_history(&self) {
        println!("Game history:");
        for (i, current_move) in self.moves.iter().enumerate() {
            let mut player;
            match i % 2 {
                0 => {
                    player = 'o';
                }
                1 => {
                    player = 'x';
                }
                _ => {
                    // TODO: handle error
                    player = ' ';
                }
            }

            if current_move.new_row.is_none() {
                println!("{}: {}{} ", player, current_move.col, current_move.row);
            } else {
                println!(
                    "{}: {}{}->{}{}",
                    player,
                    current_move.col,
                    current_move.row,
                    current_move.new_col.unwrap(),
                    current_move.new_row.unwrap()
                );
            }
        }
    }

    pub fn print_current_board(&self) {
        // Print current board on cli
        println!("  a   b   c");
        for (i, row) in self.current_state.board.iter().enumerate() {
            print!("{} ", i + 1);
            for (j, &cell) in row.iter().enumerate() {
                let char_rep = match cell {
                    1 => 'o',
                    2 => 'x',
                    _ => ' ',
                };
                if j == row.len() - 1 {
                    print!("{}\n", char_rep);
                } else {
                    print!("{} {} ", char_rep, "-".red());
                }
            }
            if i == 0 {
                println!("  {}", "| \\ | / |".red());
            } else if i == 1 {
                println!("  {}", "| / | \\ |".red());
            } else {
                println!();
            }
        }
    }

    pub fn register_move(&mut self, _move: &Move) {
        // before updating current state, save it to history
        self.state_history.push(self.current_state.clone());
        // apply on current board first
        let (row_coord, col_coord) = _move.new_as_coord();
        // new piece
        if _move.is_new_move() {
            self.current_state.board[row_coord][col_coord] = self.current_state.turn;
        }
        // moving piece
        else {
            self.current_state.board[row_coord][col_coord] = 0;
            let (new_row_coord, new_col_coord) = _move.move_as_coord();
            self.current_state.board[new_row_coord][new_col_coord] = self.current_state.turn;
        }

        match self.current_state.turn {
            1 => {
                if self.current_state.player_one_remaining > 0 {
                    self.current_state.player_one_remaining -= 1
                };
            }
            2 => {
                if self.current_state.player_two_remaining > 0 {
                    self.current_state.player_two_remaining -= 1
                };
            }
            _ => (),
        }

        println!("Move has been registered:");
        // record Move to move history
        self.moves.push(_move.clone());
        self.print_current_board();
    }

    pub fn update_next_state(&mut self) {
        match self.current_state.turn {
            1 => {
                self.current_state.turn = 2;
            }
            2 => {
                self.current_state.turn = 1;
            }
            _ => (),
        }
    }

    pub fn check_win(&self) -> u8 {
        let board = &self.current_state.board;
        let mut game_over = false;
        for i in 0..3 {
            if (board[i][0] != 0 && board[i][0] == board[i][1] && board[i][0] == board[i][2])
                || (board[0][i] != 0 && board[0][i] == board[1][i] && board[0][i] == board[2][i])
            {
                self.write_history();
                game_over = true;
            }
        }

        // Check diagonals
        if (board[0][0] != 0 && board[0][0] == board[1][1] && board[0][0] == board[2][2])
            || (board[0][2] != 0 && board[0][2] == board[1][1] && board[0][2] == board[2][0])
        {
            self.write_history();
            game_over = true;
        }

        if game_over {
            return self.current_state.turn;
        } else {
            return 0;
        };
    }

    pub fn validate_move(&self, _move: &Move) -> Result<(), GameError> {
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
                return Err(GameError::PlaceOccupied {
                    row: row_coord,
                    col: col_coord,
                });
            }
            match self.current_state.turn {
                1 => {
                    if self.current_state.player_one_remaining == 0 {
                        return Err(GameError::MaxPiecePlayed {
                            player: current_player,
                        });
                    }
                }
                2 => {
                    if self.current_state.player_two_remaining == 0 {
                        return Err(GameError::MaxPiecePlayed {
                            player: current_player,
                        });
                    }
                }
                _ => {
                    return Err(GameError::CustomError {
                        message: "unknown turn.",
                    })
                }
            }
            return Ok(());
        }

        // moving existing piece
        // validate if piece exists to move
        if self.current_state.board[row_coord][col_coord] == 0 {
            return Err(GameError::NoPieceToMove {
                row: row_coord,
                col: col_coord,
            });
        }
        // validate if all remaining pieces are played.
        if self.current_state.player_one_remaining > 0
            || self.current_state.player_two_remaining > 0
        {
            return Err(GameError::NotPlayedAllPieces {
                player: current_player,
            });
        }
        // validate if initial position belongs to player
        if self.current_state.turn != self.current_state.board[row_coord][col_coord] {
            return Err(GameError::IncorrectOwnership {
                player: current_player,
                row: row_coord,
                col: col_coord,
            });
        }
        // try unwrapping new row and new columns
        // convert new col and row to cartesian coordinates
        let (new_row_coord, new_col_coord) = _move.move_as_coord();
        // validate if new position is not occupied
        if self.current_state.board[new_row_coord][new_col_coord] != 0 {
            return Err(GameError::PlaceOccupied {
                row: new_row_coord,
                col: new_col_coord,
            });
        }
        // player can only move to connected grid.
        if !Referee::is_valid_move(row_coord, col_coord, new_row_coord, new_col_coord) {
            return Err(GameError::InvalidMove {});
        }

        return Ok(());
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
            return Err("Invalid row argument!");
        };
        let valid_cols = vec!['a', 'b', 'c', 'A', 'B', 'C'];
        if !valid_cols.contains(&col) {
            return Err("Invalid column argument!");
        };

        Ok((col, row))
    }

    pub fn validate_input(&self, input: Vec<&str>) -> Result<Move, &'static str> {
        // validate_input takes input vector and outputs Move
        // column is from a to c, row is from 1 to 3
        if input.len() > 2 {
            return Err("Incorrect amount of arguments.");
        }

        // check col and row from second argument
        let row;
        let col;
        match self.convert_str_to_row_col(input[0]) {
            Ok((parsed_col, parsed_row)) => {
                row = parsed_row;
                col = parsed_col;
            }
            Err(err) => return Err(err),
        }

        // New move.
        if input.len() == 1 {
            return Ok(Move {
                col: col,
                row: row,
                new_col: None,
                new_row: None,
            });
        };

        // check new col and new row
        let new_row;
        let new_col;
        // check col and row from second argument
        match self.convert_str_to_row_col(input[1]) {
            Ok((parsed_col, parsed_row)) => {
                new_row = parsed_row;
                new_col = parsed_col;
            }
            Err(err) => return Err(err),
        }

        Ok(Move {
            col: col,
            row: row,
            new_col: Some(new_col),
            new_row: Some(new_row),
        })
    }
}

fn main() {
    let mut game = Game::new();
    game.start();
}
