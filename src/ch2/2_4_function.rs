use std::fmt::Debug;

pub fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("函数参数", Box::new(|| function_args())),
        ("函数返回", Box::new(|| function_ret())),
        ("特殊返回", Box::new(|| function_ret_sp())),
        ("发散函数", Box::new(|| diverge_function())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn function_args() {
    another_function(5, 6.6);
}

fn another_function(x: i32, y: f64) {
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
}

fn function_ret() {
    let x = plus_five(6);
    println!("the value of x is {}", x);

    let y = plus_or_minus(7);
    println!("the value of y is {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

/// # 特殊返回
fn function_ret_sp() {
    let numbers = [1, 2, 3, 4, 5];
    report(numbers);
    report("Hello");

    let mut my_str = String::from("Hello");
    clear(&mut my_str);
    println!("the value of my_srt is {}", my_str);
}

/// # 隐式返回  unit type
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

/// # 显示返回 unit type
fn clear(text: &mut String) -> () {
    *text = String::from("")
}

fn diverge_function() {
    forever();
    dead_end()
}

fn dead_end() -> ! {
    panic!("wrong!")
}

fn forever() {
    loop {
        println!("{}", "Hello");
        panic!()
    }
}