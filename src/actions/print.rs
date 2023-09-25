use super::{sequential::SequentialAction, Action};

pub struct Print(pub String);

impl Action for Print {
    fn execute(&self) {
        println!("{}", self.0);
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