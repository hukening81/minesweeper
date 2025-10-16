use crate::{
    app::GameImageSource,
    data::{CellData, CellPos, CellRenderState, RoundData, RoundEndingType, RoundState},
};

pub struct Cell {
    data: CellData,
    round_state_type: RoundState,
    image_source: GameImageSource,
}
impl Cell {
    fn new(data: CellData, round_state_type: RoundState, image_source: GameImageSource) -> Self {
        Self {
            data,
            round_state_type,
            image_source,
        }
    }
}

impl egui::Widget for Cell {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        match &self.round_state_type {
            RoundState::NotStarted => ui.image(self.image_source.cell_closed),
            RoundState::Playing => match self.data.render_state {
                CellRenderState::Revealed => {
                    ui.image(self.image_source.cell_num[self.data.nearby_mines].clone())
                }
                CellRenderState::Covered => {
                    if self.data.is_flagged {
                        ui.image(self.image_source.cell_flag)
                    } else {
                        ui.image(self.image_source.cell_closed)
                    }
                }
            },
            RoundState::Ended(round_ending_type) => match round_ending_type.clone() {
                crate::data::RoundEndingType::ClickedMine(cell_pos) => {
                    if !self.data.is_mine {
                        ui.image(self.image_source.cell_num[self.data.nearby_mines].clone())
                    } else {
                        if cell_pos == self.data.position {
                            ui.image(self.image_source.cell_mine_red)
                        } else {
                            ui.image(self.image_source.cell_mine)
                        }
                    }
                }
                crate::data::RoundEndingType::Victory => {
                    ui.image(self.image_source.cell_num[self.data.nearby_mines].clone())
                }
            },
        }
    }
}
pub struct GameBoard<'a> {
    round_state: &'a mut RoundData,
    rect: egui::Rect,
}
impl<'a> GameBoard<'a> {
    pub fn new(round_state: &'a mut RoundData, rect: egui::Rect) -> Self {
        Self { round_state, rect }
    }
}
impl egui::Widget for GameBoard<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        // let global_state: crate::data::GlobalState =
        //     ui.data(|d| d.get_temp(egui::Id::NULL)).unwrap();
        let image_source: crate::app::GameImageSource =
            ui.data(|d| d.get_temp(egui::Id::new("IMAGE_SOURCE")).unwrap());

        let cell_size = egui::vec2(
            self.rect.width() / self.round_state.board_size as f32,
            self.rect.width() / self.round_state.board_size as f32,
        );

        egui::Grid::new("GameBoard")
            .show(ui, |ui| {
                for j in 0..self.round_state.board_size {
                    for k in 0..self.round_state.board_size {
                        // ui.add_sized(cell_size, egui::Label::new("11"));
                        ui.add_sized(
                            cell_size,
                            Cell::new(
                                self.round_state.board_data.cells[j][k].clone(),
                                self.round_state.round_state_type.clone(),
                                image_source.clone(),
                            ),
                            // egui::Label::new(format!("({},{})",j,k))
                        );
                        let response = ui.interact(
                            egui::Rect::from_min_size(
                                egui::pos2(
                                    self.rect.min.x + cell_size.x * k as f32,
                                    self.rect.min.y + cell_size.y * j as f32,
                                ),
                                cell_size,
                            ),
                            egui::Id::from(format!("{j},{k}")),
                            egui::Sense::click(),
                        );
                        if (self.round_state.round_state_type == RoundState::NotStarted)
                            || self.round_state.round_state_type == RoundState::Playing
                        {
                            if response.clicked_by(egui::PointerButton::Primary) {
                                self.handle_left_click(&CellPos::new(j, k));
                            }
                            if response.clicked_by(egui::PointerButton::Secondary) {
                                self.handle_right_click(&CellPos::new(j, k));
                            }
                        }
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
impl GameBoard<'_> {
    fn handle_right_click(&mut self, pos: &CellPos) {
        let board_data = &self.round_state.board_data;
        let mut cell = board_data.cells[pos.x][pos.y].clone();
        if cell.render_state == CellRenderState::Covered {
            cell.is_flagged = !cell.is_flagged;
            self.round_state.board_data.update_cells(vec![cell]);
        }
    }
    fn handle_left_click(&mut self, pos: &CellPos) {
        let board_data = &self.round_state.board_data;
        let origin_cell = &board_data.cells[pos.x][pos.y];
        match origin_cell.render_state {
            CellRenderState::Covered => {
                self.reveal_cell(pos);
            }
            CellRenderState::Revealed => {
                if let Some((last_click_pos, last_click_time)) = &board_data.last_click {
                    if last_click_pos.clone() == pos.clone()
                        && *last_click_time - chrono::Utc::now()
                            < chrono::TimeDelta::milliseconds(250)
                        && origin_cell.nearby_mines > 0
                    {
                        let surround_cells =
                            self.round_state.board_data.get_surround_cells(pos).clone();
                        if origin_cell.nearby_mines
                            == surround_cells.iter().filter(|it| it.is_flagged).count()
                        {
                            self.reveal_nearby_cell(pos);
                        }
                    }
                }
            }
        }
        self.round_state.board_data.last_click = Some((pos.clone(), chrono::Utc::now()));
    }
    fn reveal_cell(&mut self, pos: &CellPos) {
        let mut cell = self.round_state.board_data.get_cell(pos);

        if cell.render_state == CellRenderState::Revealed {
            return;
        }

        if cell.is_mine {
            self.round_state.round_state_type = crate::data::RoundState::Ended(
                crate::data::RoundEndingType::ClickedMine(pos.clone()),
            );
        } else {
            if self.round_state.round_state_type == RoundState::NotStarted {
                self.round_state.start_time = chrono::Utc::now().timestamp() as u32;
                self.round_state.round_state_type = RoundState::Playing;
            }

            cell.render_state = CellRenderState::Revealed;
            self.round_state.board_data.update_cells(vec![cell.clone()]);
            if cell.nearby_mines == 0 {
                self.reveal_nearby_cell(pos);
            }
        }
    }
    fn reveal_nearby_cell(&mut self, pos: &CellPos) {
        let surround_cells = self.round_state.board_data.get_surround_cells(pos).clone();
        for it in &surround_cells {
            if !it.is_flagged {
                self.reveal_cell(&it.position);
            }
        }
    }
}
