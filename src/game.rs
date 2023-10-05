use crate::{element::Element, scene::Scene, scene_registry::SceneRegistry, GAME_INSTANCE};

#[derive(Debug, Default)]
pub struct Game {
    pub scene_registry: SceneRegistry,
    game_state: GameState,
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
    /// retrieves the element if existent
    pub fn get_element_by_name(&self, element: impl Into<String>) -> Option<&Element> {
        let el = element.into();
        let current_scene = self
            .game_state
            .current_scene
            .hotspots
            .iter()
            .filter(|value| el.to_lowercase() == value.name.to_lowercase())
            .collect::<Vec<&Element>>();
        return current_scene.get(0).cloned();
    }
    pub fn get_current_scene(&self) -> &Scene {
        &self.game_state.current_scene
    }
}

#[derive(Debug, Default)]
pub struct GameState {
    pub current_scene: Scene,
}

#[cfg(test)]
mod tests {
    use crate::{
        element::{Element, ElementType},
        scene::Scene,
        GAME_INSTANCE,
    };

    use super::Game;
    fn setup_instance() {
        let scene = Scene {
            hotspots: vec![
                Element {
                    id: "element.test_element1",
                    name: "TestElement1",
                    r#type: ElementType::InEnvironment,
                },
                Element {
                    id: "element.test_element2",
                    name: "TestElement 2",
                    r#type: ElementType::InEnvironment,
                },
            ],
            id: "scene.test_scene",
            name: "TestScene",
        };
        GAME_INSTANCE.set(Game::new(scene)).unwrap();
    }

    #[test]
    fn test_get_element_by_name() {
        setup_instance();
        let _testscene1 = Element {
            id: "element.test_element1",
            name: "TestElement1",
            r#type: ElementType::InEnvironment,
        };
        let _testscene2 = Element {
            id: "element.test_element2",
            name: "TestElement 2",
            r#type: ElementType::InEnvironment,
        };

        assert!(Game::global().get_element_by_name("").is_none());
        assert!(Game::global().get_element_by_name("RandomName").is_none());
        assert!(matches!(
            Game::global().get_element_by_name("TestElement1"),
            Some(_testscene1)
        ));
        assert!(matches!(
            Game::global().get_element_by_name("testelement1"),
            Some(_testscene1)
        ));
        assert!(matches!(
            Game::global().get_element_by_name("testelement 2"),
            Some(_testscene2)
        ));
    }
}
