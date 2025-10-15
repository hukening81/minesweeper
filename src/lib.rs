#![warn(clippy::all, rust_2018_idioms)]

pub mod action;
pub mod app;
pub mod constants;
pub mod data;
pub mod game_logic;
pub mod scenes;
pub mod utils;
pub mod widgets;
pub use app::MineSweeper;
