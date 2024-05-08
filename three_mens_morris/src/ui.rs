use eframe::egui;

use crate::types::{OngoingGame, Move};

use crate::stupid_bot::StupidBot;

pub struct GameGUI {
    // column, and row
    moves: Vec<Move>,
    state_history: Vec<OngoingGame>,
    current_state: OngoingGame,
    winner: char,
    player_mode: u8,
    bot: StupidBot,
}

impl Default for GameGUI {
    fn default() -> Self {
        Self {
            moves: Vec::new(),
            state_history: Vec::new(),
            current_state: OngoingGame::new(),
            winner: ' ',
            player_mode: 2,
            bot: StupidBot::new(),
        }
    }
}

impl eframe::App for GameGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.heading("Rules:");
                ui.monospace("1. Each player gets 3 pieces to play");
                ui.monospace("2. Each player takes turn playing all their pieces first.");
                ui.monospace("3. When there is no piece left to play, player can move pieces to connected and unoccupied coordinates.");
                ui.monospace("4. When a player has three pieces in one line, that player wins.");
                ui.monospace("5. When a player has no more piece to move, that player loses.");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let mut positions = Vec::new();
                for row in 0..3 {
                    ui.add_space(50.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                        for col in 0..3 {
                            ui.add_space(50.0);
                            let response =  ui.add(
                                egui::Button::new(format!(
                                    "{:?}",
                                    self.current_state.board[row][col]
                                ))
                                .min_size(egui::Vec2::new(50.0, 50.0)),
                            );
                            if response.clicked() {
                                self.current_state.board[row][col] += 1;
                            }
                            positions.push(response.rect.center());
                        }
                    });
                };
                ui.painter().add(egui::Shape::line_segment(
                    [positions[0], positions[2]],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                ));
                ui.painter().add(egui::Shape::line_segment(
                    [positions[0], positions[6]],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                ));
                ui.painter().add(egui::Shape::line_segment(
                    [positions[0], positions[8]],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                ));
                ui.painter().add(egui::Shape::line_segment(
                    [positions[2], positions[6]],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                ));
                ui.painter().add(egui::Shape::line_segment(
                    [positions[2], positions[8]],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                ));
                ui.painter().add(egui::Shape::line_segment(
                    [positions[6], positions[8]],
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                ));
            });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
