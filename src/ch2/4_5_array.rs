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

    // 字符串类型 重复生成
    // 基本类型才有 copy
    // let f = [String::from("hello"); 8]; // the trait `Copy` is not implemented for `String`
    // println!("{:#?}", f);

    let f: [String; 8] = std::array::from_fn(|_i| String::from("hello,rust"));
    dbg!(f);
}