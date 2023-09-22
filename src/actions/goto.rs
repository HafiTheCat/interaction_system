use super::Action;
struct Goto {}
impl Action for Goto {
  const ACTION_ID: &'static str = "action.goto";
}