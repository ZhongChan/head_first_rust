fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("if 分支控制", Box::new(|| if_ctl())),
        ("else if 分支控制", Box::new(|| else_if_ctl())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn if_ctl() {
    let condition = true;
    let number = if condition {
        5
    } else {
        // "x"   //Type mismatch [E0308] expected `i32`, but found `&str`
        6
    };
    println!("the value of number is {}", number);
}

/// else if
fn else_if_ctl() {
    let n = 12;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }
}