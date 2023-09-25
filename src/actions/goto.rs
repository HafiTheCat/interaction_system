use super::Action;
struct Goto {}
impl Action for Goto {
  fn execute(&self) {
    todo!()
}

fn then(self, next: Box<dyn Action>) -> Box<dyn Action> {
    todo!()
}
}