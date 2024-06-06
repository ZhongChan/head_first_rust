fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本用法", Box::new(|| basic())),
        ("数组切片", Box::new(|| array_slice())),
        ("综合使用", Box::new(|| summary())),
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

/// # 数组切片
/// 面的数组切片 slice 的类型是&[i32]，与之对比，
/// 数组的类型是[i32;5]，简单总结下切片的特点
/// * 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
/// * 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
/// * 切片类型[T]拥有不固定的大小，而切片引用类型&[T]则具有固定的大小，
/// 
/// 因为 Rust 很多时候都需要固定大小数据类型，因此&[T]更有用,&str字符串切片也同理
fn array_slice() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    dbg!(slice);
}

fn summary() {
    // 编译器自动推导类型
    let one = [1, 2, 3];
    // 显示标注
    let two: [u8; 3] = [1, 2, 3];
    let blank = [0; 3];
    dbg!(blank);
    let blank1: [u8; 3] = [0; 3];
    dbg!(blank1);

    // 二维数组
    let arrays: [[u8; 4]; 3] = [[3; 4]; 3];
    dbg!(arrays);
    let arrays2: [[u8; 3]; 4] = [one, two, blank, blank1];
    dbg!(arrays2);

    // 借用
    for x in &arrays2 {
        println!("{:?}", x);

        // 使用 x 的迭代器
        for y in x.iter() {
            println!("{}", y);
        }

        let mut sum = 0;
        // 0..x.len() 语法糖，快速生成 x数组的下标
        for i in 0..x.len() {
            sum = sum + x[i];
        }
        dbg!(sum);
    }
}