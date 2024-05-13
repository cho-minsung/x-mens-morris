use eframe::egui::debug_text::print;
use three_mens_morris::{stupid_bot::{self, StupidBot}};
use three_mens_morris::types::{OngoingGame, Move};

#[test]
fn test_place_random_new_piece_as_player_one() {
    let stupid_bot = StupidBot::new();
    let mut state = OngoingGame{
        _id: String::from("f5715476-8808-498e-aa3e-d9c48487b602"),
        player_one: stupid_bot.get_id(),
        player_two: String::from("90663371-5330-45bd-90d2-41dd2496ab1e"),
        whose_turn: stupid_bot.get_id(),
        player_one_remaining: 3,
        player_two_remaining: 3,
        board: [[0; 3]; 3],
        moves: Vec::new(),
    };
    let _ = stupid_bot.place_random_new_piece(&mut state);

    assert_eq!(state.player_one_remaining, 2);
    assert!(!state.moves.is_empty());
    assert_ne!(state.board, [[0; 3]; 3]);
}

#[test]
fn test_place_random_new_piece_as_player_two() {
    let stupid_bot = StupidBot::new();
    let mut state = OngoingGame{
        _id: String::from("f5715476-8808-498e-aa3e-d9c48487b602"),
        player_one: String::from("90663371-5330-45bd-90d2-41dd2496ab1e"),
        player_two: stupid_bot.get_id(),
        whose_turn: stupid_bot.get_id(),
        player_one_remaining: 2,
        player_two_remaining: 3,
        board: [[0; 3]; 3],
        moves: Vec::new(),
    };
    state.moves.push(Move { col: 'c', row: 3, new_col: None, new_row: None });
    state.board[1][1] = 1;
    let expected_board_b4 = state.board.clone();
    let _ = stupid_bot.place_random_new_piece(&mut state);

    println!("{}", state.player_two_remaining);
    println!("{}", state.moves.len());
    println!("{}", state.flatten_board());
    assert_eq!(state.player_two_remaining, 2);
    assert_eq!(state.moves.len(), 2);
    assert_ne!(state.board, expected_board_b4);
}