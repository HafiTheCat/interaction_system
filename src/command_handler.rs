use crate::{
    game::Game,
    interaction::{Inspectable, Interactable, Interaction},
};

/// Evaluates a command and performs the resulting interaction
///
/// When the user issues a command it consists of a command and a value part.
/// Both have to be checked if valid and if so converted into an Interaction.
/// This interaction is then executed which calls the Interactions event handlers
///
///```
/// //Register the
/// handle_command("ins hotspot text2 text3")
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
    println!("Incoming Interaction: {interaction:?}");
    let action = match interaction {
        Interaction::Interact(el) => el.on_interact(),
        Interaction::Inspect(el) => el.on_inspect(),
    };
    action.execute()
}

/// converts a tupel of (cmd,value) into a respective interaction
fn get_interaction(raw_interaction: RawInteraction) -> Option<Interaction> {
    let Some(element) = Game::global().get_element_by_name(raw_interaction.value) else {
        return None;
    };
    //retrieve value
    match raw_interaction.cmd.as_str() {
        "interact" | "int" => Some(Interaction::Interact(element)),
        "inspect" | "ins" => Some(Interaction::Inspect(element)),
        _ => None,
    }
}

/// seperates the command into a tupel (cmd, value)
fn seperate_cmd<T>(input: T) -> Option<RawInteraction>
where
    T: Into<String>,
{
    let binding = <T as std::convert::Into<String>>::into(input);

    let mut binding = binding.split(' ');
    let Some(cmd) = take_first_two_from_iter(&mut binding) else {
        return None;
    };
    if cmd.0.is_empty() || cmd.1.is_empty() {
        return None;
    }
    Some(RawInteraction {
        cmd: cmd.0.to_owned(),
        value: cmd.1.join(" "),
    })
}

/// Takes the first two elements of an iterator and puts them into a tupel
/// "abc abc abc" => ("abc",["abc","abc"])
fn take_first_two_from_iter<T>(it: &mut impl Iterator<Item = T>) -> Option<(T, Vec<T>)> {
    let Some(first) = it.next() else {
        //Iterator does not have at least two elements.
        return None;
    };

    let rest = it.collect::<Vec<T>>();
    Some((first, rest))
}

#[derive(Debug)]
struct RawInteraction {
    cmd: String,
    value: String,
}

#[cfg(test)]
mod tests {

    use crate::command_handler::*;

    #[test]
    fn test_seperate_cmd() {
        std::env::set_var("RUST_BACKTRACE", "1");
        assert!(seperate_cmd("").is_none());
        assert!(seperate_cmd("a").is_none());

        let _a_b = RawInteraction {
            cmd: String::from("a"),
            value: String::from("b"),
        };
        assert!(matches!(seperate_cmd("a b"), Some(_a_b)));

        let _a_bc_de = RawInteraction {
            cmd: String::from("a"),
            value: String::from("bc de"),
        };
        assert!(matches!(seperate_cmd("a bc de"), Some(_a_bc_de)));

        let _a_b_c_d = RawInteraction {
            cmd: String::from("a"),
            value: String::from("b c d"),
        };
        assert!(matches!(seperate_cmd("a b c d"), Some(_a_b_c_d)));
    }
    #[test]
    fn test_take_first_two_from_iter() {
        let _no_element: Vec<u32> = vec![];
        let _one_element: Vec<u32> = vec![0];
        let _two_elements: Vec<u32> = vec![1, 0];
        let _three_elements: Vec<u32> = vec![2, 1, 0];
        let _four_elements: Vec<u32> = vec![3, 2, 1, 0];
        // [] => None
        assert!(take_first_two_from_iter(&mut _no_element.iter()).is_none());
        // [0] => Some((0,[]))
        assert!(matches!(
            take_first_two_from_iter(&mut _one_element.iter()),
            Some((0, _no_element))
        ));
        // [1,0] => Some((1,[0]))
        assert!(matches!(
            take_first_two_from_iter(&mut _two_elements.iter()),
            Some((1, _one_element))
        ));
        // [2,1,0] => Some(2,[1,0])
        assert!(matches!(
            take_first_two_from_iter(&mut _three_elements.iter()),
            Some((2, _two_elements))
        ));
        // [3,2,1,0] => Some(3,[2,1,0])
        assert!(matches!(
            take_first_two_from_iter(&mut _four_elements.iter()),
            Some((3, _three_elements))
        ));
    }
}
