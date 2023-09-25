use super::Action;
struct InventoryChange {}
impl Action for InventoryChange {
  fn execute(&self) {
    todo!()
}

fn then(self, next: Box<dyn Action>) -> Box<dyn Action> {
    todo!()
}
}