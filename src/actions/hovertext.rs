use super::Action;
struct HoverText {}
impl Action for HoverText {
  fn execute(&self) {
    todo!()
}

fn then(self, next: Box<dyn Action>) -> Box<dyn Action> {
    todo!()
}
}
