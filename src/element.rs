use std::fmt::{self, Debug, Display, Formatter};

use crate::{
    actions::{print::Print, Action},
    interaction::{Inspectable, Interactable},
};
trait TElement: Display {}

///Represents the type of an element
#[derive(Debug)]
pub enum ElementType {
    InEnvironment,
}
// #[allow(unreachable_patterns)]
impl Display for ElementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let icon = match self {
            ElementType::InEnvironment => "()",
        };
        write!(f, "{icon}")
    }
}

///Represents an element in a scene
#[derive(Debug)]
pub struct Element {
    pub id: &'static str,
    pub name: &'static str,
    pub r#type: ElementType,
}

impl TElement for Element {}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Interactable for Element {
    fn on_interact(&self) -> Box<dyn Action> {
        Box::new(Print("Interaction".to_string()))
    }
}

impl Inspectable for Element {
    fn on_inspect(&self) -> Box<dyn Action> {
        Box::new(Print("Inspection".to_string()))
    }
}
