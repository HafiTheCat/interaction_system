use super::Action;

pub struct Dialogue {}
impl Action for Dialogue {
    const ACTION_ID: &'static str = "action.dialogue";
}
