use std::cmp::Ordering;
use std::fs::File;
use std::io::{Error, Read};
use std::ops::Add;
use num::abs;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("结构体泛型", Box::new(|| struct_generics())),
        ("枚举泛型", Box::new(|| enum_generics())),
        ("方法中使用泛型", Box::new(|| method_generics())),
        ("const 泛型", Box::new(|| const_generics())),
        ("TryInto 安全转换", Box::new(|| try_into())),
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

/// # 方法中使用泛型
fn method_generics() {
    let p = PointGeneric::new(-1.0, -2.0);
    println!("{}", p.x);

    let p1 = PointGeneric2 { x: 1.0, y: 2.0 };
    let p2 = PointGeneric2::new("Hello", 'c');
    let p3 = p1.mix_up(p2);
    dbg!(p3);

    let point_f64 = Point { x: 1.0, y: 2.0 };
    println!("distance of two point: {}", point_f64.distance());
}

#[allow(dead_code)]
struct PointGeneric<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> PointGeneric<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl<T> PointGeneric<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct PointGeneric2<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointGeneric2<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn mix_up<V, W>(self, other: PointGeneric2<V, W>) -> PointGeneric2<T, W> {
        PointGeneric2 {
            x: self.x,
            y: other.y,
        }
    }
}

/// 为具体的泛型类实现方法
impl Point<f64, f64> {
    fn distance(&self) -> f64 {
        abs(self.x - self.y)
    }
}

fn const_generics() {
    let array_wrapper = ArrayWrapper::new([1, 2, 3]);
    println!("Element at index 2: {:?}", array_wrapper.get(2));
}


struct ArrayWrapper<T, const N: usize> {
    data: [T; N],
}


impl<T, const N: usize> ArrayWrapper<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

fn try_into() {
    let large_number: i32 = 1000;
    let smaller_number: u8 = match large_number.try_into() {
        Ok(num) => num,
        Err(e) => {
            println!("错误转换：{}", e);
            return;
        }
    };

    println!("成功转换：{}", smaller_number);
}