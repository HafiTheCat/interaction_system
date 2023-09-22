mod dialogue;
mod goto;
mod hovertext;
mod inventorychange;
mod scenechange;
pub trait Action {
    const ACTION_ID: &'static str;
}
