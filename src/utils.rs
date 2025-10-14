pub fn calculate_content_display_size(window_size: egui::Vec2) -> (f32, f32) {
    (
        window_size.y * crate::constants::WINDOW_RATIO,
        window_size.y,
    )
}
