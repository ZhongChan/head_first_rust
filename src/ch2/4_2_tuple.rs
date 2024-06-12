pub fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基础操作", Box::new(|| basic())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

/// # 元组是多种类型组合形成，因此是复合类型
/// 长度固定、顺序固定
fn basic() {
    let tup: (i32, f64, u8) = (10, 32.1, 1);

    //模式匹配结构元组
    let (x, y, z) = tup;
    println!("x:{},y:{},z:{}", x, y, z);

    //快速访问
    let ten = tup.0;
    let point_one = tup.1;
    let one = tup.2;
    println!("{},{},{}", ten, point_one, one);

    //多值返回
    let s1 = String::from("hello,rust!");
    let (s2, len) = cal_length(s1);
    println!("s2:{},len:{}", s2, len);
}

fn cal_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}