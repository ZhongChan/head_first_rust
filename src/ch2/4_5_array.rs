fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本用法", Box::new(|| basic())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn basic() {
    let a = [1, 2, 3, 4, 5];
    dbg!(a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    dbg!(months);

    // 声明类型
    let b: [i32; 6] = [1, 2, 3, 4.0 as i32, 5, 6];
    dbg!(b);

    // 快速初始化
    let c = [3; 10];
    dbg!(c); // 10 个 3

    let d = ['a'; 10];
    dbg!(d);

    let e = [10, 9, 8, 7, 6];
    let ten = e[0];
    dbg!(ten);
}