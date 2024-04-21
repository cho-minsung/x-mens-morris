pub struct SmartBot {}

impl SmartBot {
    // TODO: add a case where no moves can be made to be the winning case.
    fn make_move(&self, state: &State) -> Result<Move, ()> {
        // determine if new piece is needed
        let new_move_needed: bool = match state.turn {
            1 => {
                if state.player_one_remaining > 0 {
                    true
                }
                else {
                    false
                }
            },
            2 => {
                if state.player_two_remaining > 0 {
                    true
                }
                else {
                    false
                }
            },
            _ => {false}
        };
        
        match self.get_critical_position(state) {
            Ok(vector) => {
                if vector.is_empty() {
                    // no immediate move to make. make a random move.
                    match new_move_needed {
                        true => {
                            let next_move = Move::place_new_piece(col, row);
                            return Ok(next_move);
                        },
                        false => {
                    }
                }
                for (player, row, col) in vector {
                    // make winning move
                    if player == state.turn {
                        // make a winning move
                        match new_move_needed {
                            true => {
                                let next_move = Move::place_new_piece(col, row);
                                return Ok(next_move);
                            },
                            false => {
                                // iter through all possible move and pick which will allow to make critical move and pick at random if there are multiple moves possible.
                                let possible_moves = Referee::get_valid_moves(player, row, col, &state);
                                // no possible move is able to be made. Make a random move.
                                if possible_moves.is_empty() {
                                    return Err(());
                                }
                                // pick a random possible move from the vector
                                let mut rng = rand::thread_rng();
                                let item = possible_moves.choose(&mut rng);
                                match item {
                                    Some((new_row, new_col)) => {
                                        let next_move = Move::move_piece(row, col, *new_row, *new_col);
                                        return Ok(next_move)
                                    },
                                    None => return Err(()),
                                }
                            }
                        }
                    }
                    // make a critical defense move
                    else {

                    }
                }
            },
            Err(()) => {
                // make a random move
            }
        }
        Err(())
    }
}