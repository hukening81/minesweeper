use crate::data::CellData;
use crate::data::CellPos;
use crate::data::GameBoardData;
use rand::seq::SliceRandom as _;

pub fn generate_new_board(board_size: usize, total_mine: usize) -> GameBoardData {
    let mut mine_map = vec![false; board_size * board_size];
    let slice = &mut mine_map[0..total_mine];
    slice.fill(true);
    let mut rng = rand::rng();
    mine_map.shuffle(&mut rng);
    let mine_map: Vec<Vec<bool>> = mine_map
        .chunks(board_size)
        .map(|slice| slice.to_vec())
        .collect();

    // println!("Mine Map:{:?}",mine_map);

    let max_index= board_size-1;
    let mut result = vec![vec![CellData::default(); board_size]; board_size];

    for j in 0..board_size {
        for k in 0..board_size {
            result[j][k].is_mine = mine_map[j][k];
            result[j][k].position = CellPos::new(j, k);
            if !mine_map[j][k] {
                continue;
            }
            if j > 0 {
                result[j - 1][k].nearby_mines += 1;
                if k > 0 {
                    result[j - 1][k - 1].nearby_mines += 1;
                }
                if k < max_index{
                    result[j - 1][k + 1].nearby_mines += 1;
                }
            }
            if j < max_index{
                result[j + 1][k].nearby_mines += 1;
                if k > 0 {
                    result[j + 1][k - 1].nearby_mines += 1;
                }
                if k < max_index{
                    result[j + 1][k + 1].nearby_mines += 1;
                }
            }
            if k > 0 {
                result[j][k - 1].nearby_mines += 1;
            }
            if k < max_index{
                result[j][k + 1].nearby_mines += 1;
            }
        }
    }
    let board_data = GameBoardData {
        cells: result,
        last_click: None,
    };
    // board_data.show_mine_location();
    // board_data.show_game_board();
    board_data
}