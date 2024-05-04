use crate::state::State;
use crate::move_def::Move;
use crate::referee::Referee;

pub struct StupidBot {
    player_id: String,
    player: u8,
}

impl StupidBot {
    pub fn new() -> StupidBot {
        // player is default to 1
        return StupidBot{
            player_id: String::from("036d2541-b81f-40f9-baf6-8cd8a1d589c9"),
            player: 1,
        };
    }

    pub fn set_player(&mut self, value: u8) {
        // You can add any validation or transformation here
        self.player = value;
    }

    pub fn get_player(&self) -> u8 {
        // You can add any validation or transformation here
        return self.player;
    }

    pub fn make_random_move(&self, state: &State) -> Result<Move, ()> {
        let mut make_new_move: bool = false;
        match self.player {
            1 => {
                if state.player_one_remaining != 0 {
                    make_new_move = true;
                }
            },
            2 => {
                if state.player_two_remaining != 0 {
                    make_new_move = true;
                }
            }
            _ => return Err(())
        }
        if make_new_move {
            match Referee::get_random_new_coord(state) {
                Ok((new_row, new_col)) => return Ok(Move::place_new_piece(&new_row, &new_col)),
                Err(()) => return Err(()),
            }
        }
        
        // make a random move
        match Referee::get_random_move(&self.player, state) {
            Ok((old_row, old_col, new_row, new_col)) => {
                return Ok(Move::move_piece(&old_row, &old_col, &new_row, &new_col));
            },
            Err(()) => return Err(())
        }
    }
}