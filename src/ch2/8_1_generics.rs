use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, Read};
use std::ops::Add;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("结构体泛型", Box::new(|| struct_generics())),
        ("枚举泛型", Box::new(|| enum_generics())),
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

/// # 结构体泛型
fn struct_generics() {
    let p1 = Point { x: 3.0, y: "hello".to_string() };
    dbg!(p1);

    let p2 = Point2 { x: 3.0, y: 4.0 };
    dbg!(p2);
}

#[derive(Debug)]
#[allow(dead_code)]
/// 不能的泛型参数
struct Point<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point2<T> {
    x: T,
    y: T,
}

/// # 枚举中使用泛型
/// 两个最常用的枚举泛型:
/// null 和 错误处理
/// ```
/// enum Option<T> {
///      Some(T),
///      None,
///  }
///
/// enum Result<T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ```
fn enum_generics() {
    match read_file_to_string("test.json") {
        Ok(contents) => {
            println!("{}", contents);
        }
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
}

fn read_file_to_string(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
