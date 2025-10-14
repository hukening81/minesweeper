use egui::Widget;

use crate::{
    data::RoundState,
    widgets::{GameBoard, GameStatusBar},
};
pub struct GameScene<'a> {
    render_origin: egui::Pos2,
    width: f32,
    height: f32,
    round_state: &'a mut RoundState,
}
impl<'a> GameScene<'a> {
    pub fn new(
        render_origin: egui::Pos2,
        width: f32,
        height: f32,
        round_state: &'a mut RoundState,
    ) -> Self {
        Self {
            render_origin,
            width,
            height,
            round_state,
        }
    }
}

impl Widget for GameScene<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let global_state: crate::data::GlobalState =
            ui.data(|d| d.get_temp(egui::Id::NULL)).unwrap();

        egui::Frame::NONE
            .show(ui, |ui| {
                ui.put(
                    egui::Rect::from_min_size(
                        self.render_origin,
                        egui::vec2(self.width, self.height - self.width),
                    ),
                    GameStatusBar::new(self.round_state.clone()),
                );
                ui.put(
                    egui::Rect::from_min_size(
                        egui::pos2(
                            self.render_origin.x,
                            self.render_origin.y + (self.height - self.width),
                        ),
                        egui::vec2(self.width, self.width),
                    ),
                    GameBoard::new(
                        egui::pos2(self.render_origin.x, self.height - self.width),
                        self.width,
                        self.round_state,
                    ),
                )
            })
            .response
    }
}
