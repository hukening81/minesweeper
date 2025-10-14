use rand::seq::SliceRandom;


pub fn calculate_new_window_size(
    last_window_size: egui::Vec2,
    current_window_size: egui::Vec2,
) -> egui::Vec2 {
    let x_delta = current_window_size.x - last_window_size.x;
    let y_delta = current_window_size.y - last_window_size.y;
    println!("{:?}x{:?}", x_delta, y_delta);
    if x_delta.abs() > y_delta.abs() {
        return egui::vec2(
            current_window_size.x,
            current_window_size.x / crate::constants::WINDOW_RATIO,
        );
    } else {
        return egui::vec2(
            current_window_size.y * crate::constants::WINDOW_RATIO,
            current_window_size.y,
        );
    };
}
pub fn calculate_content_display_size(window_size: egui::Vec2) -> (f32, f32) {
    return (
        window_size.y * crate::constants::WINDOW_RATIO,
        window_size.y,
    );
}
