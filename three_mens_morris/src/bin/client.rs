use eframe::egui;

use three_mens_morris::types::OngoingGame;

pub struct Client {
    current_state: OngoingGame,
    winner: char,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            current_state: OngoingGame::new(),
            winner: ' ',
        }
    }
}

impl eframe::App for Client {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
        .frame(egui::Frame::default().inner_margin(20.0).outer_margin(20.0))
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.heading("Rules:");
                ui.monospace("1. Each player gets 3 pieces to play");
                ui.monospace("2. Each player takes turn playing all their pieces first.");
                ui.monospace("3. When there is no piece left to play, player can move pieces to connected and unoccupied coordinates.");
                ui.monospace("4. When a player has three pieces in one line, that player wins.");
                ui.monospace("5. When a player has no more piece to move, that player loses.");
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(20.0).outer_margin(20.0))
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    let mut positions = Vec::new();
                    for row in 0..3 {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                            for col in 0..3 {
                                let response = ui.add(
                                    egui::Button::new(format!(
                                        "{:?}",
                                        self.current_state.board[row][col]
                                    ))
                                    .min_size(egui::Vec2::new(50.0, 50.0)),
                                );
                                if col != 2 {
                                    ui.add_space(50.0);
                                }
                                if response.clicked() {
                                    self.current_state.board[row][col] += 1;
                                }
                                positions.push(response.rect.center());
                            }
                        });
                        ui.add_space(50.0);
                    }
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
            });
    }
}


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Three mens morris client",
        options,
        Box::new(|cc| {
            // Use the dark theme
            // cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Box::<Client>::default()
        }),
    )
}
