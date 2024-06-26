#[allow(unused_imports)]
use crate::ch2::ref_or_borrow::dangle_ref;

pub fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("悬垂引用和生命周期", Box::new(|| dangle_ref_lifetime())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}


/// 在 Rust 中，“引用”和“借用”是密切相关的概念，但它们并不完全是同一个意思。以下是对这两个术语的解释：
/// ## 引用（Reference）
/// 引用是指向某个值的指针。Rust 中的引用有两种类型：
/// * 不可变引用 (&T)：允许你读取数据但不能修改数据。
/// * 可变引用 (&mut T)：允许你读取和修改数据。
///
/// 引用的主要目的是为了避免数据的复制，通过引用可以在多个地方访问同一个数据。
///
/// ## 借用（Borrowing）
/// 借用是指将某个值的引用传递给另一个作用域。借用可以是不可变借用或可变借用：
/// * 不可变借用：当你通过不可变引用访问数据时，称之为不可变借用。一个值可以有多个不可变借用。
/// * 可变借用：当你通过可变引用访问数据时，称之为可变借用。一个值在任意时刻只能有一个可变借用，且不能与不可变借用同时存在。
///
/// ## 借用规则
/// 借用规则由 Rust 编译器强制执行，以确保数据的安全性和防止数据竞争（data race）。具体规则包括：
/// * 在任意时刻，只能有一个可变引用，或者任意数量的不可变引用。
/// * 引用必须始终有效。
///
fn basic() {
    let x = 5;

    // 不可变引用
    let r1 = &x;
    let r2 = &x;
    println!("r1: {}, r2 :{}", r1, r2); // 允许多个不可变引用

    // 可变引用
    let mut y = 6;
    let r3 = &mut y;
    println!("r3:{}", r3);
}

/// # 悬垂引用
/// 悬垂引用（dangling reference）指的是一个引用指向了已经被释放或移除的内存位置
/// # 参考
/// - [`dangle_ref`]
///
/// # 生命周期
/// 生命周期的主要作用是避免悬垂引用，它会导致程序引用了本不该引用的数据
fn dangle_ref_lifetime() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("{}", r);
    }
    // println!("{}", r);
}