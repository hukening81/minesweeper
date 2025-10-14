use crate::{
    app::GameImageSource,
    data::{CellData, CellPos, CellRenderState, CellState, RoundState, RoundStateType},
};

pub struct Cell {
    data: CellData,
    image_source: GameImageSource,
}
impl Cell {
    fn new(data: CellData, image_source: GameImageSource) -> Self {
        Self { data, image_source }
    }
}

impl egui::Widget for Cell {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = match self.data.render_state {
            CellRenderState::Revealed => ui.image(
                self.image_source
                    .get_num_image_source(self.data.nearby_mines),
            ),
            CellRenderState::Covered => {
                if (self.data.is_flagged) {
                    ui.image(self.image_source.flagged)
                } else {
                    ui.image(self.image_source.normal_block)
                }
            }
            CellRenderState::GameEnded(true) => ui.image(self.image_source.mine_block),
            CellRenderState::GameEnded(false) => ui.image(self.image_source.mine_exploded),
        };
        return response;
    }
}
pub struct GameBoard<'a> {
    render_origin: egui::Pos2,
    size: f32,
    round_state: &'a mut RoundState,
}
impl<'a> GameBoard<'a> {
    pub fn new(render_origin: egui::Pos2, size: f32, round_state: &'a mut RoundState) -> Self {
        Self {
            render_origin,
            size,
            round_state,
        }
    }
}
impl<'a> egui::Widget for GameBoard<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        // let global_state: crate::data::GlobalState =
        //     ui.data(|d| d.get_temp(egui::Id::NULL)).unwrap();
        let image_source: crate::app::GameImageSource =
            ui.data(|d| d.get_temp(egui::Id::new("IMAGE_SOURCE")).unwrap());

        let cell_size = egui::vec2(
            self.size / self.round_state.board_size as f32,
            self.size / self.round_state.board_size as f32,
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
                                image_source.clone(),
                            ),
                            // egui::Label::new(format!("({},{})",j,k))
                        );
                        let response = ui.interact(
                            egui::Rect::from_min_size(
                                egui::pos2(
                                    self.render_origin.x + cell_size.x * k as f32,
                                    self.render_origin.y + cell_size.y * j as f32,
                                ),
                                cell_size,
                            ),
                            egui::Id::from(format!("{},{}", j, k)),
                            egui::Sense::click(),
                        );
                        if response.clicked_by(egui::PointerButton::Primary) {
                            self.handle_left_click(&CellPos::new(j, k));
                        }
                        if response.clicked_by(egui::PointerButton::Secondary) {
                            self.handle_right_click(&CellPos::new(j, k));
                        }
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
impl<'a> GameBoard<'a> {
    fn handle_right_click(&mut self, pos: &CellPos) {
        let board_data = &self.round_state.board_data;
        let mut cell = (&board_data.cells[pos.x][pos.y]).to_owned();
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
                self.reveal_cell(&pos);
            }
            CellRenderState::Revealed => {
                if let Some((last_click_pos, last_click_time)) = &board_data.last_click {
                    if last_click_pos.clone() == pos.clone()
                        && last_click_time.clone() - chrono::Utc::now()
                            < chrono::TimeDelta::milliseconds(250)
                        && origin_cell.nearby_mines > 0
                    {
                        let surround_cells = self
                            .round_state
                            .board_data
                            .get_surround_cells(pos)
                            .to_owned();
                        if origin_cell.nearby_mines
                            == surround_cells.iter().filter(|it| it.is_flagged).count() as u8
                        {
                            self.reveal_nearby_cell(pos);
                        }
                    }
                }
            }
            CellRenderState::GameEnded(_) => {}
        }
        self.round_state.board_data.last_click = Some((pos.clone(), chrono::Utc::now()));
    }
    fn reveal_cell(&mut self, pos: &CellPos) {
        let mut cell = self.round_state.board_data.get_cell(pos);

        if cell.render_state == CellRenderState::Revealed {
            return;
        }

        if cell.is_mine {
            panic!("YOU HIT A MINE at ({},{})", pos.x, pos.y);
        } else {
            cell.render_state = CellRenderState::Revealed;
            self.round_state
                .board_data
                .update_cells(vec![cell.to_owned()]);
            if cell.nearby_mines == 0 {
                // println!(
                //     "Revealing near by cells from ({},{})",
                //     cell.position.x, cell.position.y
                // );
                self.reveal_nearby_cell(pos)
            }
        }
    }
    fn reveal_nearby_cell(&mut self, pos: &CellPos) {
        let surround_cells = self
            .round_state
            .board_data
            .get_surround_cells(pos)
            .to_owned();
        surround_cells.iter().for_each(|it| {
            println!(
                "Revealing cell ({},{}), request by ({},{}),nearby mines count: {}, requested is mine?{}",
                it.position.x,
                it.position.y,
                pos.x,
                pos.y,
                self.round_state.board_data.get_cell(pos).nearby_mines,
                it.is_mine,

            );
            if (!it.is_flagged) {
                self.reveal_cell(&it.position)
            }
        });
    }
    fn reveal_non_flagged_cell(&mut self, pos: &CellPos) {}
}
