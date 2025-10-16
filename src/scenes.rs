pub mod game;
pub use game::GameScene;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum SceneType {
    Game,
    Setting,
    RoomSetting,
    PlayerStanding,
}