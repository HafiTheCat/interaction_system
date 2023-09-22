

use crate::element::Element;

#[derive(Default, Debug)]
pub struct Scene {
    pub id: &'static str,
    pub name: &'static str,
    pub hotspots: Vec<Element>,
}

// impl Display for Scene {
//   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//     write!(f, "{}", self.)
//   }
// }
