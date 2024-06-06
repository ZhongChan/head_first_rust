#[macro_use]
extern crate head_first_rust;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("语句和表达式", Box::new(|| statements_and_expressions())),
        ("表达式", Box::new(|| expressions())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

/// # 语句和表达式
/// * Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值
/// * 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值，因此在上述函数体的三行代码中，前两行是语句，最后一行是表达式。
///
/// * 对于 Rust 语言而言，这种基于语句（statement）和表达式（expression）的方式是非常重要的，
/// * 你需要能明确的区分这两个概念, 但是对于很多其它语言而言，这两个往往无需区分。
/// * 基于表达式是函数式语言的重要特征，表达式总要返回值。
fn statements_and_expressions() {
    add_with_extra(9, 11);
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;//语句
    let y = y + 5;//语句
    x + y //表达式
}

fn expressions() {
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (c, d) = ("hi", false);
    //let b = (let a = 8);  语句不能赋值给其他值
    print_size_of_val!(a);
    print_size_of_val!(b);
    print_size_of_val!(c);
    print_size_of_val!(d);

    //语句块
    let y = {
        let x = 3;
        x + 1
    };
    print_size_of_val!(y);

    assert_eq!(ret_unit_type(), ());
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块是表达式，可以用于赋值
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    print_size_of_val!(y);
}

