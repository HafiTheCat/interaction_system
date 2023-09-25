use super::Action;

pub struct Dialogue {
    text: String,
}
impl Action for Dialogue {
    fn execute(&self) {
        println!("{}", self.text);
    }
}
