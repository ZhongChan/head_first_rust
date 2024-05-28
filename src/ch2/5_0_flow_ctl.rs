fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("if 分支控制", Box::new(|| if_ctl())),
        ("else if 分支控制", Box::new(|| else_if_ctl())),
        ("for 循环", Box::new(|| for_loop())),
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
/// 有一点要注意，就算有多个分支能匹配，也只有第一个匹配的分支会被执行！
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

fn for_loop() {
    for x in 1..5 {
        println!("{}", x);
    }

    let owner_ship_arr = ["x".to_string(), "y".to_string(), "z".to_string()];
    for x in owner_ship_arr {
        println!("{}", x);
    }
    // println!("{:?}", owner_ship_arr); //  borrow of moved value: `owner_ship_arr`

    let str = ["x", "y", "z"];
    for x in str {
        println!("{}", x);
    }
    println!("{:?}", str); // &str 实现了 Copy
}