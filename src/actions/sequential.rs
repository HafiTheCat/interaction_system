use super::Action;

pub struct SequentialAction {
    pub(crate) actions: Vec<Box<dyn Action>>,
}

impl Action for SequentialAction {
    fn execute(&self) {
        for action in &self.actions {
            action.execute();
        }
    }

    fn then(mut self, next: Box<dyn Action>) -> Box<dyn Action>
    where
        Self: Sized,
    {
        self.actions.push(next);
        Box::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::{print::Print, sequential::SequentialAction, Action};

    #[test]
    fn name() {
        let action_sequence = SequentialAction {
            actions: vec![
                Box::new(Print("Step 1".to_string())),
                Box::new(Print("Step 2".to_string())),
                Box::new(Print("Step 3".to_string())),
            ],
        };

        action_sequence.execute();
    }
}
