use super::Action;
struct HoverText {}
impl Action for HoverText {
  const ACTION_ID: &'static str = "action.hovertext";
}
