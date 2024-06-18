use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    println!("{:?}", your_name);

    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_the_in = false;
    for visitor in visitor_list {
        if your_name == visitor {
            allow_the_in = true;
            break;
        }
    }

    if allow_the_in {
        println!("Welcome!");
    } else {
        println!("Sorry,yor are not in the list")
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to readline");
    your_name.trim().to_lowercase()
}
