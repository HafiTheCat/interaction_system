use crate::element::Element;

#[derive(Default, Debug)]
pub struct Scene {
    pub id: &'static str,
    pub name: &'static str,
    pub hotspots: Vec<Element>,
}
