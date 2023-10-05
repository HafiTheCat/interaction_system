mod actions;
mod command_handler;
mod element;
mod game;
mod interaction;
mod scene;
mod scene_registry;
mod graveyard;
use std::io;

use game::Game;
use once_cell::sync::OnceCell;

use crate::{
    command_handler::handle_command,
    element::{Element, ElementType},
    scene::Scene,
};


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

fn main() -> io::Result<()> {
    init();
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear(); // First clear the String. Otherwise it will keep adding to it
        print_scene_info();
        print_actions();
        io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string
        handle_command(input_string.trim().to_lowercase().clone());
    }
    Ok(())
}

/// prints info of the current scene and hotspots
fn print_scene_info() {
    let current_scene = Game::global().get_current_scene();
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