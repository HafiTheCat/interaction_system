use super::{sequential::SequentialAction, Action};

pub struct Dialogue {
    text: String,
}
impl Action for Dialogue {
    fn execute(&self) {
        println!("{}", self.text);
    }

    fn then(self, next: Box<dyn Action>) -> Box<dyn Action>
    where
        Self: Sized,
    {
        Box::new(SequentialAction {
            actions: vec![next],
        })
    }
}
