use self::sequential::SequentialAction;

mod dialogue;
mod goto;
mod hovertext;
mod inventorychange;
pub mod print;
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

