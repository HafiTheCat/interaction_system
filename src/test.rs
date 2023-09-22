// pub trait TaskMC {
//     const TEST: &'static str;
// }

// pub fn create<MC, E>(mm: &ModelManager, data: E) -> Result<f32,()>
// where
//     MC: TaskMC,
//     E: Task,
// {
// let t = mm.get_instance();
// let fields = data.task_method();
// let MC::TEST;
//     return Ok(0_f32);
// }

// struct ModelController {}
// impl TaskMC for ModelController {
//     const TEST: &'static str = "task";
// }

// impl ModelController {
//     pub fn create(mm: &ModelManager, task: ExampleTask) -> Result<f32, ()> {
//         crate::test::create::<Self, _>(mm, task)
//     }
//     pub fn get(mm: &ModelManager, task: ExampleTask) -> Result<(), ()> {
//       Ok(())
//     }
//     pub fn update() -> Result<(), ()> {
//       Ok(())
//     }
//     pub fn delete() -> Result<(), ()> {
//       Ok(())
//     }
// }

// #[derive(Clone)]
// pub struct ModelManager {
//     instance: String,
// }

// impl ModelManager {
//     // Constructor
//     pub fn new() -> Result<Self, ()> {
//         Ok(ModelManager {
//             instance: String::from("[object Object]"),
//         })
//     }

//     /// Returns a reference
//     pub(in crate) fn get_instance(&self) -> &str {
//         &self.instance
//     }
// }

// trait Task {
//   fn task_method(self);
// }

// struct ExampleTask {
//   field:String
// }
// impl Task for ExampleTask{
//   fn task_method(self) {
//     dbg!("Example_task test");
//     dbg!(self.field);
//   }
// }


// fn do_sth() {
//     let mm = ModelManager::new().unwrap();
//     let task = ExampleTask {
//       field:String::from("Example_task")
//     };
//     let id = ModelController::create(&mm,task);
    
// }
