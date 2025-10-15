#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod constants;
pub mod scenes;
pub mod data;
pub mod utils;
pub mod widgets;
pub mod game_logic;
pub use app::MineSweeper;
