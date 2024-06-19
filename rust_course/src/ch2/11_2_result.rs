use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let result = File::open("foo.txt");
    let f = result.unwrap_or_else(|err| match err.kind() {
        ErrorKind::NotFound => File::create("foo.txt").unwrap_or_else(|err| {
            panic!("Problem creating file: {:?}", err)
        }),

        other_err => {
            panic!("Problem opening file: {:?}", other_err)
        }
    });
    println!("Is File: {}", f.metadata().unwrap().is_file());

    // let bar = File::open("bar.txt").unwrap();
    // println!("Is File: {}", bar.metadata().unwrap().is_file());

    let bar = File::open("bar.txt").expect("bar.txt not found");
    println!("Is File: {}", bar.metadata().unwrap().is_file());
}