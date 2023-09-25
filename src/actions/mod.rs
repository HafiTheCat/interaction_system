use self::sequential::SequentialAction;

mod dialogue;
mod goto;
mod hovertext;
mod inventorychange;
mod print;
mod scenechange;
mod sequential;

pub trait Action {
    fn execute(&self);
    fn then(self, next: Box<dyn Action>) -> Box<dyn Action>
    where
        Self: Sized,
    {
        Box::new(SequentialAction {
            actions: vec![next],
        })
    }
}

// Dialogue::new()
// Goto::new()11
// HoverText::new()
// InventoryChange::new()
// SceneChange::new()
// Schedule::new()

// Dialogue::new().then(Goto::new())
