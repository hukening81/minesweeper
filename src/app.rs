use std::cell;

use crate::data::{GlobalState, RoundData};
use egui::ImageSource;
use log::debug;

#[derive(Clone)]
pub struct GameImageSource {
    pub cell_num: Vec<egui::ImageSource<'static>>,
    pub cell_num_yellow: Vec<egui::ImageSource<'static>>,
    pub cell_closed: egui::ImageSource<'static>,
    pub cell_flag: egui::ImageSource<'static>,
    pub cell_mine: egui::ImageSource<'static>,
    pub cell_mine_red: egui::ImageSource<'static>,
}
impl GameImageSource {}
impl Default for GameImageSource {
    fn default() -> Self {
        let mut cell_num = vec![];
        cell_num.push(egui::include_image!("../assets/game/type0.svg"));
        cell_num.push(egui::include_image!("../assets/game/type1.svg"));
        cell_num.push(egui::include_image!("../assets/game/type2.svg"));
        cell_num.push(egui::include_image!("../assets/game/type3.svg"));
        cell_num.push(egui::include_image!("../assets/game/type4.svg"));
        cell_num.push(egui::include_image!("../assets/game/type5.svg"));
        cell_num.push(egui::include_image!("../assets/game/type6.svg"));
        cell_num.push(egui::include_image!("../assets/game/type7.svg"));
        cell_num.push(egui::include_image!("../assets/game/type8.svg"));

        let mut cell_num_yellow = vec![];
        cell_num_yellow.push(egui::include_image!("../assets/game/type0.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type1_yellow.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type2_yellow.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type3_yellow.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type4_yellow.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type5_yellow.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type6_yellow.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type7_yellow.svg"));
        cell_num_yellow.push(egui::include_image!("../assets/game/type8_yellow.svg"));

        Self {
            cell_num,
            cell_num_yellow,
            cell_flag: egui::include_image!("../assets/game/flag.svg"),
            cell_closed: egui::include_image!("../assets/game/closed.svg"),
            cell_mine: egui::include_image!("../assets/game/mine.svg"),
            cell_mine_red: egui::include_image!("../assets/game/mine_red.svg"),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct MineSweeper {
    global_state: GlobalState,
    round: RoundData,
    #[serde(skip)]
    image_sources: GameImageSource,
}

impl MineSweeper {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // } else {
        //     Default::default()
        // }
        Default::default()
    }
    pub fn stop_game() {}
}

impl eframe::App for MineSweeper {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Global Style
        ctx.style_mut(|style| {
            style.spacing.window_margin = egui::Margin::ZERO;
            style.spacing.item_spacing = egui::Vec2::ZERO;
            style.visuals.window_stroke = egui::Stroke::NONE;
            // style.debug.show_widget_hits = true;
            // style.debug.debug_on_hover = true;
            style.spacing.interact_size = egui::Vec2::ZERO;
            style.spacing.indent = 0.0;
        });

        // Insert nessecery data for nested UI to render
        ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, self.global_state.clone()));
        ctx.data_mut(|d| {
            d.insert_temp(egui::Id::new("IMAGE_SOURCE"), self.image_sources.clone());
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(198, 198, 198)))
            .show(ctx, |ui| {
                // // Resize Logic
                let current_window_size = ui.available_size();
                let last_window_size = egui::vec2(
                    self.global_state.window_size.0,
                    self.global_state.window_size.1,
                );
                if current_window_size != last_window_size {
                    self.global_state.content_size =
                        crate::utils::calculate_content_display_size(current_window_size);
                    ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(egui::vec2(
                        self.global_state.content_size.0,
                        500.0,
                    )));
                    self.global_state.window_size = (current_window_size.x, current_window_size.y);
                }

                debug!("{:?}", self.global_state);

                // Padding two side panels
                let padding_size =
                    ((self.global_state.window_size.0 - self.global_state.content_size.0) / 2.0)
                        .max(0.0);

                // Render Main Scene
                // Calculate size for different sections
                self.global_state.layout_state.scene_panel_height =
                    self.global_state.content_size.1 * 0.95;
                self.global_state.layout_state.function_panel_height =
                    self.global_state.content_size.1
                        - self.global_state.layout_state.scene_panel_height;
                self.global_state.layout_state.global_x_padding = padding_size;

                let main_scene_rect = egui::Rect::from_min_size(
                    egui::pos2(padding_size, 0.0),
                    egui::vec2(
                        self.global_state.content_size.0,
                        self.global_state.layout_state.scene_panel_height,
                    ),
                );

                let function_panel_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        padding_size,
                        self.global_state.layout_state.scene_panel_height,
                    ),
                    egui::vec2(
                        self.global_state.content_size.0,
                        self.global_state.content_size.1,
                    ),
                );

                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        let main_scene = match &self.round.round_state_type {
                            crate::data::RoundState::NotStarted
                            | crate::data::RoundState::Playing => {
                                crate::scenes::GameScene::new(main_scene_rect, &mut self.round)
                            }
                            crate::data::RoundState::Ended(round_ending_type) => {
                                crate::scenes::GameScene::new(main_scene_rect, &mut self.round)
                            }
                        };
                        let main_scene = ui.put(main_scene_rect, main_scene);
                        let function_panel = crate::widgets::FunctionPanel::new();
                        ui.put(function_panel_rect, function_panel);
                    })
            });
        ctx.request_repaint();
    }
}
