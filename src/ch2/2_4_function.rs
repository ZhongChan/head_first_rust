use std::fmt::Debug;
use head_first_rust::generate_main;

generate_main!(
    ("函数参数", function_args),
    ("函数返回", function_ret),
    ("特殊返回", function_ret_sp),
    ("发散函数", diverge_function)
);

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