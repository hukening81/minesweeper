use egui::Widget;

use crate::{
    data::RoundState,
    widgets::{GameBoard, GameStatusBar},
};
pub struct GameScene<'a> {
    rect: egui::Rect,
    round_state: &'a mut RoundState,
}
impl<'a> GameScene<'a> {
    pub fn new(rect: egui::Rect, round_state: &'a mut RoundState) -> Self {
        Self { rect, round_state }
    }
}

impl Widget for GameScene<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let global_state: crate::data::GlobalState =
            ui.data(|d| d.get_temp(egui::Id::NULL)).unwrap();
        self.round_state.update_round_state();

        let status_bar_rect = egui::Rect::from_min_size(
            self.rect.min,
            egui::vec2(self.rect.width(), self.rect.height() - self.rect.width()),
        );

        let game_board_rect = egui::Rect::from_min_size(
            egui::pos2(
                self.rect.min.x,
                self.rect.min.y + (self.rect.height() - self.rect.width()),
            ),
            self.rect.size(),
        );

        egui::Frame::NONE
            .show(ui, |ui| {
                ui.put(
                    status_bar_rect,
                    GameStatusBar::new(&self.round_state, status_bar_rect),
                );
                ui.put(
                    game_board_rect,
                    GameBoard::new(self.round_state, game_board_rect),
                )
            })
            .response
    }
}
