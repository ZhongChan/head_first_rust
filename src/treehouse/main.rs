use std::io::stdin;
use crate::VisitorAction::{Accept, AcceptWithNote, Probation, Refuse};

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse.", Accept, 19),
        Visitor::new("steve", "Hello Steve, your milk is in the fridge.", AcceptWithNote {
            note: "Lactose-free milk in the fridge".to_string()
        }, 15),
        Visitor::new("fred", "Wow, who invited Fred?", Refuse, 21),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let your_name = what_is_your_name();
        println!("{:?}", your_name);
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == your_name);

        match known_visitor {
            None => {
                if your_name.is_empty() {
                    println!("The final list of visitors:");
                    println!("{:#?}", visitor_list);
                    break; // to loop end
                } else {
                    println!("{} is not on the visitor list.", your_name);
                    visitor_list.push(Visitor::new(&your_name, "New friend", Probation, 20));
                }
            }
            Some(visitor) => match &visitor.action {
                Accept => {
                    println!("Welcome to the treehouse!");
                    visitor.greet_visitor()
                }
                AcceptWithNote { note } => { println!("{}", note) }
                Probation => {
                    println!("todo!")
                }
                _ => { println!("Go away!") }
            }
        }
    }
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: u8,
}

impl Visitor {
    pub fn new(name: &str, greeting: &str, action: VisitorAction, age: u8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting)
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to readline");
    your_name.trim().to_lowercase()
}
