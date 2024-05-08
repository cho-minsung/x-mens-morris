use crate::types::OngoingGame;

pub struct Referee {
    // Referee is always fair and right!
}

impl Referee {
    pub fn get_checkmate_positions(state: &OngoingGame) -> Result<Vec<(u8, usize, usize)>, ()> {
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
}
