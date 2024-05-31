use std::cmp::Ordering;
use std::ops::Add;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn basic() {
    println!("{} + {} = {}", 10, 9, add_i8(10, 9));
    println!("{} + {} = {}", 10, 9, add_i32(10, 9));
    println!("{} + {} = {}", 10, 9, add_i64(10, 9));

    println!("{}", add(2.0, 3.0));
    println!("{}", add(2, 3));

    let number_list = vec![10, 30, 25, 60, 80, 8];
    println!("The largest number is: {}", largest(&number_list));

    let char_list = vec!['d', 'f', 'a', 'z', 't'];
    println!("The largest char is: {}", largest(&char_list));
    println!("{:?}", char_list);
}

fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}

fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

fn add<T: Add<Output=T>>(a: T, b: T) -> T {
    a + b
}


fn largest<T1: PartialOrd + Copy>(list: &[T1]) -> T1 {
    let mut largest = list[0];
    for &item in list.iter() {
        if largest.partial_cmp(&item) == Some(Ordering::Less) {
            largest = item;
        }
    }
    largest
}