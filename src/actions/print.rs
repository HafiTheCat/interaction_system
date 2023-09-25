use super::Action;

pub struct Print(pub String);

impl Action for Print {
    fn execute(&self) {
        println!("{}", self.0);
    }
}