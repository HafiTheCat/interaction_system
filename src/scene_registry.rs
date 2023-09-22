use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};
pub trait TSceneRegistry {
    fn add_scene(self, id: impl Into<String>) -> Self;
    fn remove_scene(self, id: impl Into<String>) -> Self;
}

#[derive(Default, Debug)]
pub struct SceneRegistry {
    scenes: HashSet<String>,
}

impl SceneRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TSceneRegistry for SceneRegistry {
    fn add_scene(mut self, id: impl Into<String>) -> Self {
        self.scenes.insert(id.into());
        self
    }

    fn remove_scene(mut self, id: impl Into<String>) -> Self {
        self.scenes.remove(&id.into());
        self
    }
}

impl Display for SceneRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scenes.iter().join(", "))
    }
}
