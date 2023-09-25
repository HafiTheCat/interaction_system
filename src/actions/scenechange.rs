use super::Action;
struct SceneChange {}
impl Action for SceneChange {

    fn execute(&self) {
        todo!()
    }

    fn then(self, next: Box<dyn Action>) -> Box<dyn Action> {
        todo!()
    }

}