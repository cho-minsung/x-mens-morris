use crate::state::State;

use rand::prelude::SliceRandom;

pub struct Referee {
    // Referee is always fair and right!
}

impl Referee {
    pub fn get_checkmate_positions(state: &State) -> Result<Vec<(u8, usize, usize)>, ()> {
        // inputs current state and output coordinates of urgent defense and their owners
        // ok if does not need defense. Err if needed.
        let mut critical_moves: Vec<(u8, usize, usize)> = Vec::new();
        let mut critical_row;
        let mut critical_col;

        let board = &state.board;

        // opponent will win if any diagonal or horizontal line has two pieces and is not blocked by the bot.
        // and you will win if you have two of your pieces in a line.
        for player in [1, 2] {
            // iter through row
            for i in 0..3 {
                // check row
                let count_row = board[i].iter().filter(|&&x| x == player).count();
                if count_row == 2 && board[i].iter().any(|&x| x == 0) {
                    critical_row = i as usize;
                    critical_col = board[i].iter().position(|&x| x == 0).unwrap();
                    critical_moves.push((player, critical_row, critical_col));
                }

                // Check columns
                let count_col = board
                    .iter()
                    .map(|row| row[i])
                    .filter(|&x| x == player)
                    .count();
                if count_col == 2 && board.iter().any(|row| row[i] == 0) {
                    critical_row = board.iter().position(|row| row[i] == 0).unwrap();
                    critical_col = i as usize;
                    critical_moves.push((player, critical_row, critical_col));
                }
            }

            // Check diagonals
            let diag_left = (0..3).map(|i| board[i][i]).filter(|&x| x == player).count();
            let diag_right = (0..3)
                .map(|i| board[i][2 - i])
                .filter(|&x| x == player)
                .count();
            if diag_left == 2 && (0..3).any(|i| board[i][i] == 0) {
                // coordinates are the same for left diagonal.
                critical_row = (0..3).position(|i| board[i][i] == 0).unwrap();
                critical_moves.push((player, critical_row, critical_row));
            }
            if diag_right == 2 && (0..3).any(|i| board[i][2 - i] == 0) {
                critical_row = (0..3).position(|i| board[i][2 - i] == 0).unwrap();
                critical_col = 2 - (0..3).position(|i| board[i][2 - i] == 0).unwrap();
                critical_moves.push((player, critical_row, critical_col));
            }
        }

        if critical_moves.is_empty() {
            return Err(());
        }
        Ok(critical_moves)
    }

    pub fn get_random_move(player: &u8, state: &State) -> Result<(usize, usize, usize, usize), ()> {
        // return a random move a select player can make
        let mut all_moves: Vec<(usize, usize, usize, usize)> = Vec::new();
        // Get all pieces a select player owns
        for (old_row, row_arr) in state.board.iter().enumerate() {
            for (old_col, &piece) in row_arr.iter().enumerate() {
                if piece == *player {
                    match Referee::get_all_valid_moves(player, &old_row, &old_col, state) {
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

        match position {
            Some(&(old_row, old_col, new_row, new_col)) => {
                return Ok((old_row, old_col, new_row, new_col));
            }
            None => {
                return Err(());
            }
        }
    }

    pub fn get_all_valid_moves(
        player: &u8,
        old_row: &usize,
        old_col: &usize,
        state: &State,
    ) -> Result<Vec<(usize, usize, usize, usize)>, ()> {
        // return all possible moves from an input position for an input player
        // return error if the piece is not owned by the player
        if state.board[*old_row][*old_col] != *player {
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
        state: &State,
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

    pub fn is_valid_move(old_row: usize, old_col: usize, new_row: usize, new_col: usize) -> bool {
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
            return false;
        }

        let row_diff = (old_row as i32 - new_row as i32).abs();
        let col_diff = (old_col as i32 - new_col as i32).abs();

        row_diff <= 1 && col_diff <= 1
    }

    pub fn get_random_new_coord(state: &State) -> Result<(usize, usize), ()> {
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
            return Err(());
        }

        let mut rng = rand::thread_rng();
        let position = unoccupied_positions.choose(&mut rng);

        match position {
            Some(&(i, j)) => {
                println!("unoccupied position: {} {}", i, j);
                return Ok((i, j));
            }
            None => {
                return Err(());
            }
        }
    }
}
