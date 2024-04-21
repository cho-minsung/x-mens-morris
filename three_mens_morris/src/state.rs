#[derive(Clone)]
pub struct State {
    pub turn: u8, // 1 for 1st player, 2 for 2nd player
    pub player_one_remaining: u8,
    pub player_two_remaining: u8,
    pub board: [[u8; 3]; 3], // (row x col x 3) matrix where 0 is empty, 1 is 1st player, and 2 is 2nd player
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