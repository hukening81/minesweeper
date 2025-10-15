pub struct FunctionPanel {
}

impl Default for FunctionPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionPanel{
    pub fn new() -> Self {
        Self{}
    }
}
impl egui::Widget for FunctionPanel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::NONE
            .fill(egui::Color32::BLACK)
            .show(ui, |ui| {
                ui.label("Function Label")
            })
            .response
    }
}
