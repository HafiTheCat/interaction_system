use crate::{element::Element, game::Game, interaction::Interaction};

/// Evaluates a command and performs the resulting interaction
///
/// When the user issues a command it consists of a command and a value part.
/// Both have to be checked if valid and if so converted into an Interaction.
/// This interaction is then executed which calls the Interactions event handlers
///
///```
/// //Register the
/// handle_command("ins hotspot")
///```
///
///[even more advanced explanations if necessary]
pub fn handle_command(input: impl Into<String>) {
    let Some(interaction) = evaluate_command(input.into()) else {
        return;
    };

    perform_interaction(interaction);
}

/// evaluates a command and performs the resulting action
fn evaluate_command(input: impl Into<String>) -> Option<Interaction> {
    // if valid command (e.g. one of elements in scene)
    let r#in = input.into();
    // then execute of interaction.
    let Some(raw_interaction) = seperate_cmd(r#in) else {
        return None;
    };
    get_interaction(raw_interaction)
}

/// performs an interaction
fn perform_interaction(interaction: Interaction) {
    println!("Performed Interaction: {interaction:?}");
}

/// converts a tupel of (cmd,value) into a respective interaction
fn get_interaction(raw_interaction: RawInteraction<'_>) -> Option<Interaction> {
    let Some(element) = get_element(raw_interaction.value) else {
        return None;
    };
    //retrieve value
    match raw_interaction.cmd {
        "interact" | "int" => Some(Interaction::Interact(element)),
        "inspect" | "ins" => Some(Interaction::Inspect(element)),
        _ => None,
    }
}

/// retrieves the element if existent
fn get_element(element: impl Into<String>) -> Option<&'static Element> {
    let el = element.into();
    dbg!(&el);
    let current_scene = Game::global()
        .game_state
        .current_scene
        .hotspots
        .iter()
        .filter(|value| el == value.name.to_lowercase())
        .collect::<Vec<&Element>>();
    dbg!(&current_scene);
    return current_scene.get(0).cloned();
}

/// seperates the command into a tupel (cmd, value)
fn seperate_cmd<'a, T>(input: T) -> Option<RawInteraction<'a>>
where
    T: Into<String>,
{
    let binding = <T as std::convert::Into<String>>::into(input);
    let mut binding = binding.split(' ');
    let Some(cmd) = take_first_two_from_iter(&mut binding) else {
        return None;
    };
    Some(RawInteraction {
        cmd: cmd.0,
        value: cmd.1.join(" ").as_str(),
    })
}

/// Takes the first two elements of an iterator and puts them into a tupel
/// "abc abc abc" => ("abc")
fn take_first_two_from_iter<T>(it: &mut impl Iterator<Item = T>) -> Option<(T, &[T])> {
    let Some(first) = it.next() else {
        //Iterator does not have at least two elements.
        return None;
    };
    let rest = it.collect::<Vec<T>>();
    Some((first, rest.as_slice()))
}

struct RawInteraction<'a> {
    cmd: &'a str,
    value: &'a str,
}

// #[cfg(test)]
// mod tests {
//     use crate::command_handler::*;

//     #[test]
//     fn test_seperate_cmd() {
//         assert_eq!(seperate_cmd(&mut "".chars()), None);
//         assert_eq!(
//             seperate_cmd(&mut "command arg".split("")),
//             Some(("command", "arg"))
//         );
//         assert_eq!(seperate_cmd(&mut "ins".chars()), None);
//     }
//     // #[test]
//     // fn evaluate_command_tests() {
//     //     assert!(evaluate_command("").is_none());
//     //     assert!(evaluate_command("test").is_none());
//     //     assert!(evaluate_command("interact").is_none());
//     //     assert!(evaluate_command("inspect").is_none());
//     //     assert!(evaluate_command("int").is_none());
//     //     assert!(evaluate_command("ins").is_none());
//     //     assert!(evaluate_command("test test").is_none());

//     //     assert!(evaluate_command("interact test").is_some());
//     //     assert!(evaluate_command("inspect test").is_some());
//     //     assert!(evaluate_command("int test").is_some());
//     //     assert!(evaluate_command("ins test").is_some());
//     // }
// }
