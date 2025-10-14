pub mod game;
pub use game::GameScene;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum SceneType {
    GameScene,
}

pub fn get_scene_widget(scene_type: &SceneType) -> GameScene {
    todo!()
}
