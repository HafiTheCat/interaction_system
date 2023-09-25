mod dialogue;
mod goto;
mod hovertext;
mod inventorychange;
mod scenechange;
mod schedule;
mod sequential;
mod print;
pub trait Action {
    fn execute(&self);
    fn then(self, next: Box<dyn Action>) -> Box<dyn Action>
    where
        Self: Sized;
}

// Dialogue::new()
// Goto::new()11
// HoverText::new()
// InventoryChange::new()
// SceneChange::new()
// Schedule::new()

// Dialogue::new().then(Goto::new())