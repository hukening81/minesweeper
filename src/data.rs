use chrono::{DateTime, TimeZone as _};

use crate::constants::{DEFAULT_BOARD_SIZE, DEFAULT_MINE_AMOUNT};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GameSettings {
    pub board_size: i16,
    pub total_mines: i16,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            board_size: 16,
            total_mines: 40,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct LayoutState {
    pub scene_panel_height: f32,
    pub function_panel_height: f32,
    pub global_x_padding: f32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GlobalState {
    pub current_scene: crate::scenes::SceneType,
    pub game_settings: GameSettings,
    // pub round_state: RoundState,
    pub window_size: (f32, f32),
    pub content_size: (f32, f32),
    pub layout_state: LayoutState,
}

impl GlobalState {
    pub fn change_scene() {}
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            current_scene: crate::scenes::SceneType::Game,
            game_settings: Default::default(),
            // round_state: RoundState::default(),
            window_size: (400.0, 500.0),
            content_size: (400.0, 500.0),
            layout_state: LayoutState {
                scene_panel_height: 450.0,
                function_panel_height: 50.0,
                global_x_padding: 0.0,
            },
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub enum CellRenderState {
    Covered,
    Revealed,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CellData {
    pub position: CellPos,
    pub is_mine: bool,
    pub is_flagged: bool,
    pub nearby_mines: u8,
    pub render_state: CellRenderState,
}

impl Default for CellData {
    fn default() -> Self {
        Self {
            position: CellPos::new(0, 0),
            is_mine: false,
            is_flagged: false,
            nearby_mines: 0,
            render_state: CellRenderState::Covered,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GameBoardData {
    #[serde(skip)]
    pub last_click: Option<(CellPos, DateTime<chrono::Utc>)>,
    pub cells: Vec<Vec<CellData>>,
}

impl GameBoardData {
    pub fn show_mine_location(&self) {
        let mut text = String::new();
        let board_size = self.cells.len();
        for j in 0..board_size {
            let mut tmp_string = String::new();
            for k in 0..board_size {
                if (self.cells[j][k]).is_mine {
                    tmp_string.push_str(" X");
                } else {
                    tmp_string.push_str(" O");
                }
            }
            tmp_string.push('\n');
            text += tmp_string.as_str();
            tmp_string.clear();
        }
        println!("{text}");
    }
    pub fn show_game_board(&self) {
        let mut text = String::new();
        let board_size = self.cells.len();
        for j in 0..board_size {
            let mut tmp_string = String::new();
            for k in 0..board_size {
                if (self.cells[j][k]).is_mine {
                    tmp_string.push_str(" X");
                } else {
                    tmp_string.push_str(format!(" {}", self.cells[j][k].nearby_mines).as_str());
                }
            }
            tmp_string.push('\n');
            text += tmp_string.as_str();
            tmp_string.clear();
        }
        println!("{text}");
    }
    pub fn get_flag_count(&self) -> usize {
        let result = self
            .cells
            .iter()
            .map(|it| {
                it.iter()
                    .map(|it| i32::from(it.is_flagged))
                    .reduce(|a, b| a + b)
                    .unwrap_or(0)
            })
            .reduce(|a, b| a + b)
            .unwrap_or(0);
        result as usize
    }
    pub fn get_remain_cell_count(&self) -> usize {
        let result = self
            .cells
            .iter()
            .map(|it| {
                it.iter()
                    .map(|it| i32::from(it.render_state == CellRenderState::Covered))
                    .reduce(|a, b| a + b)
                    .unwrap_or(0)
            })
            .reduce(|a, b| a + b)
            .unwrap_or(0);
        result as usize
    }
    pub fn get_cell(&self, pos: &CellPos) -> CellData {
        self.cells[pos.x][pos.y].clone()
    }
    pub fn update_cells(&mut self, data: Vec<CellData>) {
        let board_size = self.cells.len();
        for cell in &data {
            if cell.position.x < board_size && cell.position.y < board_size {
                self.cells[cell.position.x][cell.position.y] = cell.clone();
            }
        }
    }
    pub fn get_surround_cells(&self, pos: &CellPos) -> Vec<CellData> {
        let max_index = self.cells.len() - 1;
        let mut result: Vec<CellData> = vec![];

        if pos.x > 0 {
            result.push(self.cells[pos.x - 1][pos.y].clone());
            if pos.y > 0 {
                result.push(self.cells[pos.x - 1][pos.y - 1].clone());
            }
            if pos.y < max_index {
                result.push(self.cells[pos.x - 1][pos.y + 1].clone());
            }
        }
        if pos.x < max_index {
            result.push(self.cells[pos.x + 1][pos.y].clone());
            if pos.y > 0 {
                result.push(self.cells[pos.x + 1][pos.y - 1].clone());
            }
            if pos.y < max_index {
                result.push(self.cells[pos.x + 1][pos.y + 1].clone());
            }
        }
        if pos.y > 0 {
            result.push(self.cells[pos.x][pos.y - 1].clone());
        }
        if pos.y < max_index {
            result.push(self.cells[pos.x][pos.y + 1].clone());
        }

        result
    }
}

impl Default for GameBoardData {
    fn default() -> Self {
        crate::game_logic::generate_new_board(DEFAULT_BOARD_SIZE, DEFAULT_MINE_AMOUNT)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RoundData {
    pub start_time: u32,
    #[serde(skip)]
    pub time_passed: u32,
    pub board_size: usize,
    pub total_mine: usize,
    pub flags_placed: i16,
    pub board_data: GameBoardData,
    pub round_state_type: RoundState,
    pub mines_remaining: usize,
}

impl RoundData {
    pub fn update_round_state(&mut self) {
        self.mines_remaining = self.total_mine - self.board_data.get_flag_count();
        match &mut self.round_state_type {
            RoundState::NotStarted => {}
            RoundState::Playing => {
                self.time_passed = chrono::Utc::now().timestamp() as u32 - self.start_time;
                if self.mines_remaining <= 0 {
                    self.round_state_type = RoundState::Ended(RoundEndingType::Victory);
                }
            }
            RoundState::Ended(round_ending_type) => {}
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub enum RoundEndingType {
    ClickedMine(CellPos),
    Victory,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum RoundState {
    NotStarted,
    Playing,
    Ended(RoundEndingType),
}

impl Default for RoundData {
    fn default() -> Self {
        Self {
            start_time: Default::default(),
            board_size: DEFAULT_BOARD_SIZE,
            total_mine: DEFAULT_MINE_AMOUNT,
            time_passed: 0,
            flags_placed: 0,
            board_data: GameBoardData::default(),
            round_state_type: RoundState::NotStarted,
            mines_remaining: DEFAULT_MINE_AMOUNT,
        }
    }
}

impl RoundData {
    pub fn new(board_size: usize, total_mine: usize) -> Self {
        Self {
            board_size,
            total_mine,
            flags_placed: 0,
            board_data: crate::game_logic::generate_new_board(board_size, total_mine),
            start_time: 0,
            time_passed: 0,
            round_state_type: RoundState::NotStarted,
            mines_remaining: total_mine,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CellPos {
    pub x: usize,
    pub y: usize,
}

impl CellPos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
