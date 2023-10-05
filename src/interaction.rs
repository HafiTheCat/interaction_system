use crate::{actions::Action, element::Element};

#[derive(Debug)]
pub enum Interaction {
    Interact(&'static Element),
    Inspect(&'static Element),
}

pub trait Interactable {
    fn on_interact(&self) -> Box<dyn Action>;
}

pub trait Inspectable {
    fn on_inspect(&self) -> Box<dyn Action>;
}

pub trait Hoverable {
    fn on_hover(&self) -> Box<dyn Action>;
}
