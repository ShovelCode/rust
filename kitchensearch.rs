use std::io;
use std::io::Write;

enum Action {
    Go(String),
    Open(String),
    Invalid,
}

struct Room {
    name: String,
    description: String,
}

impl Room {
    fn new(name: &str, description: &str) -> Room {
        Room {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

fn get_user_input() -> String {
    print!("What do you want to do? ");
    io::stdout().flush().unwrap();
    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("Failed to read line");
    action.trim().to_string()
}

fn parse_input(user_input: String) -> Action {
    let parts: Vec<&str> = user_input.split_whitespace().collect();
    match parts.as_slice() {
        ["go", direction] => Action::Go(direction.to_string()),
        ["open", object] => Action::Open(object.to_string()),
        _ => Action::Invalid,
    }
}

fn main() {
    let mut current_room = Room::new("Living Room", "You are in a cozy living room. There's a door leading to the kitchen.");

    loop {
        println!("{}", current_room.description);

        let action = parse_input(get_user_input());

        match action {
            Action::Go(direction) => {
                if direction == "kitchen" && current_room.name == "Living Room" {
                    current_room = Room::new("Kitchen", "You are now in the kitchen. You see a fridge.");
                } else {
                    println!("You can't go there!");
                }
            },
            Action::Open(object) => {
                if object == "fridge" && current_room.name == "Kitchen" {
                    println!("You find some leftover pizza!");
                } else {
                    println!("You can't open that!");
                }
            },
            Action::Invalid => {
                println!("I don't understand your command.")
            }
        }
    }
}
