use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    println!("{:?}", your_name);

    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hello Steve, your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    let mut allow_the_in = false;
    let mut your = Visitor::new("", "");
    for visitor in visitor_list {
        if your_name == visitor.name {
            allow_the_in = true;
            your = visitor;
            break;
        }
    }

    if allow_the_in {
        your.greet_visitor();
    } else {
        println!("Sorry,yor are not in the list");
    }
}

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    pub fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting)
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to readline");
    your_name.trim().to_lowercase()
}
