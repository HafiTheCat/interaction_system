use crate::element::Element;

#[derive(Debug)]
pub enum Interaction {
    Interact(&'static Element),
    Inspect(&'static Element),
}
