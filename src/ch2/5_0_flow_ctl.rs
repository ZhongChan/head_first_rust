fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("if 分支控制", Box::new(|| if_ctl())),
        ("else if 分支控制", Box::new(|| else_if_ctl())),
        ("for 循环", Box::new(|| for_loop())),
        ("while 循环", Box::new(|| while_loop())),
        ("loop 循环", Box::new(|| the_loop())),
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

/// # else if
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

    let a = [1, 2, 3, 4, 5];
    // k v 迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第 {} 个元素是 {}", i + 1, v);
    }

    // continue
    for x in 1..4 {
        if x == 2 {
            continue;
        }
        println!("{}", x);
    }

    for x in 1..5 {
        if x == 2 {
            break;
        }
        println!("{}", x);
    }
}

fn while_loop() {
    let mut n = 0;
    while n < 5 {
        println!("{}!", n);
        n = n + 1;
    }
    println!("我从 while 出来了啦！");

    let mut m = 0;
    loop {
        if m > 5 {
            break;
        }
        println!("{}!!", m);
        m += 1;
    }
    println!("我从 loop 出来了！");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is : {}", a[index]);
        index = index + 1;
    }

    println!("while ending!");

    for element in a.iter() {
        println!("the value is : {}", element);
    }
}


/// # loop 不要轻易使用
/// break 可以单独使用，也可以带一个返回值，有些类似 return
/// loop 是一个表达式，因此可以返回一个值
fn the_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    dbg!(result);
}