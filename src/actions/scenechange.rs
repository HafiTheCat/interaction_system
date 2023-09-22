use super::Action;
struct SceneChange {}
impl Action for SceneChange {
    const ACTION_ID: &'static str = "action.scenechange";
}