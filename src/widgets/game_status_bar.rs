use crate::data::RoundState;
pub struct GameStatusBar {}

impl GameStatusBar {
    pub fn new(state: RoundState) -> Self {
        Self {}
    }
}
impl egui::Widget for GameStatusBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("game_status_bar")
    }
}
