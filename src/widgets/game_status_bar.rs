use std::time;

use crate::data::RoundState;
pub struct PassedTimeIndicator {
    passed_time: u32,
}

impl PassedTimeIndicator {
    pub fn new(passed_time: u32) -> Self {
        Self { passed_time }
    }
}

impl egui::Widget for PassedTimeIndicator {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::NONE
            .fill(egui::Color32::WHITE)
            .show(ui, |ui| {
                ui.label(format!("{}", self.passed_time));
            })
            .response
    }
}

pub struct RemainMineIndicator {
    remain_mines: usize,
}

impl RemainMineIndicator {
    pub fn new(remain_mines: usize) -> Self {
        Self { remain_mines }
    }
}

impl egui::Widget for RemainMineIndicator {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::NONE
            .fill(egui::Color32::WHITE)
            .show(ui, |ui| {
                ui.label(format!("{}", self.remain_mines));
            })
            .response
    }
}

pub struct GameStatusBar<'a> {
    round_state: &'a RoundState,
    rect: egui::Rect,
}

impl<'a> GameStatusBar<'a> {
    pub fn new(round_state: &'a RoundState, rect: egui::Rect) -> Self {
        Self { round_state, rect }
    }
}
impl<'a> egui::Widget for GameStatusBar<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let box_height = self.rect.height() * 0.8;
        let box_width = box_height * 1.5;
        let box_margin = (self.rect.height() - box_height) / 2.0;
        let box_size = egui::vec2(box_width, box_height);

        let remain_mine_widget_rect = egui::Rect::from_min_size(
            egui::pos2(self.rect.min.x + box_margin, self.rect.min.y + box_margin),
            box_size,
        );
        ui.put(
            remain_mine_widget_rect,
            RemainMineIndicator::new(self.round_state.mines_remaining),
        );
        let time_widget_rect = egui::Rect::from_min_size(
            egui::pos2(
                self.rect.min.x + self.rect.width() - box_margin - box_width,
                self.rect.min.y + box_margin,
            ),
            box_size,
        );
        ui.put(
            time_widget_rect,
            PassedTimeIndicator::new(self.round_state.time_passed),
        )
    }
}
