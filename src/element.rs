use std::fmt::{self, Debug, Display, Formatter};
trait TElement: Display {}

///Represents the type of an element
#[derive(Debug)]
pub enum ElementType {
    InEnvironment,
    // InInventory,
    // Way,
}
// #[allow(unreachable_patterns)]
impl Display for ElementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let icon = match self {
            ElementType::InEnvironment => "()",
            // ElementType::InInventory => "[()]",
            // ElementType::Way => "==>",
            // _ => unreachable!(),
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
