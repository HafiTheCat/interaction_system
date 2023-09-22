// use scene_registry::{SceneRegistry, TSceneRegistry};

// use crate::scene_registry;

use crate::{scene::Scene, scene_registry::SceneRegistry, GAME_INSTANCE};

#[derive(Debug, Default)]
pub struct Game {
    pub scene_registry: SceneRegistry,
    pub game_state: GameState,
}

impl Game {
    pub fn new(start_scene: Scene) -> Self {
        Game {
            game_state: GameState {
                current_scene: start_scene,
            },
            scene_registry: SceneRegistry::new(),
        }
    }
    pub fn global() -> &'static Game {
        GAME_INSTANCE.get().expect("game is not initialized")
    }
    // pub fn show_hotspots(self) -> Option<Vec<String>> {
    //   let current_scene = self.game_state.current_scene;
    // }
}

#[derive(Debug, Default)]
pub struct GameState {
    pub current_scene: Scene,
}
