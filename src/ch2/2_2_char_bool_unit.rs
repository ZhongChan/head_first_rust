use head_first_rust::{print_size_of_char, print_size_of_val};

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("字符类型", Box::new(|| char_type())),
        ("布尔类型", Box::new(|| bool_type())),
        ("单元类型", Box::new(|| unit_type())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

/// # 字符类型
/// * Rust 的 char 类型可以表示任何 Unicode 标量值。
/// * 这是因为在 Rust 中，char 类型占用 4 字节（32 位），
/// * 这足以表示 Unicode 标准中的所有字符，包括那些超过了基本多语言平面（BMP）的字符。
///
/// * 这与一些其他语言（如 Java 或 C#）的 char 类型不同，
/// * 它们只占用 2 字节（16 位），
/// * 只能直接表示 Unicode 的基本多语言平面的字符。
/// * 对于超出这个范围的字符，它们需要使用特殊的编码方案，如 UTF-16。

fn char_type() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    print_size_of_char!(c);
    print_size_of_char!(z);
    print_size_of_char!(g);
    print_size_of_char!(heart_eyed_cat);
}

fn bool_type() {
    let t = true;
    let f: bool = false; //使用类型标注
    print_size_of_val!(t);
    print_size_of_val!(f);
}

/// # 单元类型
/// * 在 Rust 中，单元类型（Unit Type）是一种特殊的类型，用 () 表示。
/// * 它类似于其他编程语言中的 void 类型，但在 Rust 中，它实际上是一个类型，可以被用作函数的返回值，也可以被用作变量的类型。
///
/// * 单元类型只有一个值，也就是 ()。它通常用在不需要返回任何有意义值的函数中。例如：
///
/// # Examples
///
/// ```
/// fn print_hello() -> () {
///     println!("Hello, world!");
/// }
/// ```
fn unit_type() {
    println!("size of unit type: {} bytes", std::mem::size_of::<()>());
}