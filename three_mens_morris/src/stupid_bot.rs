use rand::prelude::SliceRandom;

use crate::referee::Referee;
use crate::types::{Move, OngoingGame};

pub struct StupidBot {
    player_id: String,
}

impl StupidBot {
    pub fn new() -> StupidBot {
        // bot is default to go first
        return StupidBot {
            player_id: String::from("036d2541-b81f-40f9-baf6-8cd8a1d589c9"),
        };
    }

    pub fn get_id(&self) -> String {
        return self.player_id.clone();
    }

    pub fn determine_piece(&self, state: &OngoingGame) -> u8 {
        // Stateless function to determine piece
        // piece is 1 if bot is first player
        match state.player_one == self.player_id {
            true => return 1,
            false => return 2,
        };
    }

    pub fn place_random_new_piece(&self, state: &mut OngoingGame) -> Result<(), ()> {
        // This function takes ongoing game state and update a new piece there.

        // If it is not bot's turn return error
        if state.whose_turn != self.player_id {
            return Err(());
        }

        // Return error when bot ID is not in the game
        if state.player_one != self.player_id && state.player_two != self.player_id {
            return Err(());
        }

        // piece is 1 if bot is first player
        let bot_piece: u8 = self.determine_piece(state);

        // Return error if bot has no piece to play
        if (state.player_one_remaining <= 0 && bot_piece == 1)
            || (state.player_two_remaining <= 0 && bot_piece == 2)
        {
            return Err(());
        }

        // Pick a random position from unoccupied coordinate
        let mut unoccupied_positions = Vec::new();
        // Find positions where the value is zero
        for (i, row) in state.board.iter().enumerate() {
            for (j, &piece) in row.iter().enumerate() {
                if piece == 0 {
                    unoccupied_positions.push((i, j));
                }
            }
        }
        if unoccupied_positions.is_empty() {
            println!("Error in finding where to put a new piece.");
            return Err(());
        }

        let mut rng = rand::thread_rng();
        let position = unoccupied_positions.choose(&mut rng).unwrap();
        let row = &position.0;
        let col = &position.1;
        let new_move = Move::convert_to_new_move(row, col).unwrap();
        println!("Bot played random position: {}", new_move.print());

        // Update state at the end
        state.update_turn();
        if bot_piece == 1 {
            state.player_one_remaining -= 1;
        }
        else {
            state.player_two_remaining -= 1;
        };
        state.board[*row][*col] = bot_piece;
        // Update the new move
        state.moves.push(new_move);

        Ok(())
    }

    pub fn get_random_valid_move(
        &self,
        state: &OngoingGame,
    ) -> Result<(usize, usize, usize, usize), ()> {
        // return a random validated move a select player can make
        let mut all_moves: Vec<(usize, usize, usize, usize)> = Vec::new();
        // Get all pieces a select player owns
        for (old_row, row_arr) in state.board.iter().enumerate() {
            for (old_col, &piece) in row_arr.iter().enumerate() {
                if piece == self.determine_piece(state) {
                    match self.get_all_valid_moves(&old_row, &old_col, state) {
                        Ok(vector) => {
                            all_moves.extend(vector);
                        }
                        Err(()) => continue,
                    }
                }
            }
        }
        if all_moves.is_empty() {
            return Err(());
        }

        let mut rng = rand::thread_rng();
        let position = all_moves.choose(&mut rng);

        // match position {
        //     Some(&(old_row, old_col, new_row, new_col)) => {
        //         // update the state
        //         // if self.player_turn
        //     }
        //     None => {
        //         return Err(());
        //     }
        // }
        Err(())
    }

    pub fn get_all_valid_moves(
        &self,
        old_row: &usize,
        old_col: &usize,
        state: &OngoingGame,
    ) -> Result<Vec<(usize, usize, usize, usize)>, ()> {
        // return all possible moves from an input position for an input player
        // return error if the piece is not owned by the player
        if state.board[*old_row][*old_col] != self.determine_piece(state) {
            return Err(());
        }
        let mut new_moves: Vec<(usize, usize, usize, usize)> = Vec::new();
        for (new_row, row_arr) in state.board.iter().enumerate() {
            for (new_col, &piece) in row_arr.iter().enumerate() {
                // it cannot be moved unless the new row and col are occupied
                if piece != 0 {
                    continue;
                }
                if Referee::is_valid_move(*old_row, *old_col, new_row, new_col) {
                    new_moves.push((*old_row, *old_col, new_row, new_col));
                }
            }
        }
        if new_moves.is_empty() {
            return Err(());
        }
        return Ok(new_moves);
    }

    pub fn get_valid_moves_to_position(
        player: u8,
        row: usize,
        col: usize,
        state: &OngoingGame,
    ) -> Vec<(usize, usize)> {
        // return all possible moves for a specific player to move to an input row and column
        // return empty vector if there is no move that can achieve this position
        let mut vector: Vec<(usize, usize)> = Vec::new();

        for new_row in 0..3 {
            for new_col in 0..3 {
                if state.board[new_row][new_col] != player {
                    continue;
                }
                if Referee::is_valid_move(new_row, new_col, row, col) {
                    vector.push((new_row, new_col));
                }
            }
        }
        return vector;
    }

    // pub fn make_random_move(&self, state: &OngoingGame) -> Result<(), ()> {
    //     // return error if it is not bot's turn
    //     let bot_id = self.get_id();

    //     // check if bot has new moves left to make
    //     let make_new_move = match self.determine_piece(state) {
    //         1 => state.player_one_remaining != 0,
    //         2 => state.player_two_remaining != 0,
    //         _ => {
    //             println!("Unknown state in checking bot's remaining moves.");
    //             return Err(());
    //         }
    //     };

    //     // make a random new move
    //     if make_new_move {
    //         return self.place_random_new_piece(state);
    //     }

    //     // // make a verified random move
    //     // match Referee::get_random_move(&self.player, state) {
    //     //     Ok((old_row, old_col, new_row, new_col)) => {
    //     //         return Ok(Move::move_piece(&old_row, &old_col, &new_row, &new_col));
    //     //     }
    //     //     Err(()) => return Err(()),
    //     // }

    //     Ok(())
    // }
}
