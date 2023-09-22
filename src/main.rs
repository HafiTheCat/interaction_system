mod actions;
mod element;
mod game;
mod interaction;
mod item_registry;
mod scene;
mod scene_registry;
mod test;
mod command_handler;
use std::io;

use game::Game;
use once_cell::sync::OnceCell;

use crate::{
    element::{Element, ElementType},
    scene::Scene, command_handler::handle_command,
};
// use once_cell::sync::Lazy;
// use std::{collections::HashMap, sync::Mutex};

// static GLOBAL_DATA: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
//     let mut m = HashMap::new();
//     m.insert("init_scene".to_string(), "test".to_string());
//     m.insert("test_key".to_owned(), "test_res".to_string());
//     Mutex::new(m)
// });
// println!("{:?}", GLOBAL_DATA.lock().unwrap());

static GAME_INSTANCE: OnceCell<Game> = OnceCell::new();

/// initializes scene and elements
fn init() {
    let scene = Scene {
        hotspots: vec![
            Element {
                id: "element.test_element1",
                name: "TestElement1",
                r#type: ElementType::InEnvironment,
            },
            Element {
                id: "element.test_element2",
                name: "TestElement2",
                r#type: ElementType::InEnvironment,
            },
        ],
        id: "scene.test_scene",
        name: "TestScene",
    };

    GAME_INSTANCE.set(Game::new(scene)).unwrap();
}

fn main() -> io::Result<()>{
    init();
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear(); // First clear the String. Otherwise it will keep adding to it
        print_scene_info();
        print_actions();
        io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string
        handle_command(input_string.trim().to_lowercase().clone());
    }
    println!("See you later!");
    Ok(())
}

/// prints info of the current scene and hotspots
fn print_scene_info() {
    let current_scene = &(Game::global().game_state.current_scene);
    println!(
        "====================================================\n\
        \tCurrent Location: {}\n\
        \tHotspots: {}\n\
        ====================================================\n",
        current_scene.name,
        current_scene
            .hotspots
            .iter()
            .fold(String::new(), |acc, arg| acc + "<" + arg.name + ">  ")
    );
}

/// prints info of the possible actions
fn print_actions() {
    print!(
        "Inspect Hotspot: \tins <hotspot>\n\
        Interact with Hotspot: \tint <hotspot>\n\
        Which action would you like to perform?\n",
    );
}


// struct Dialogue<'a> {
//     id: &'a str,
//     text: &'a str,
// }

// enum Interaction<'a> {
//     SceneChange(&'a str),
//     Dialogue(&'a Dialogue<'a>),
// }

// fn getDialog() -> Vec<(&'static str, &'static str)> {
//     vec![
//         ("test.dialog.1", "this is a test dialog"),
//         ("test.dialog.2", "THIS IS A TEXT DIALOG"),
//     ]
// }

