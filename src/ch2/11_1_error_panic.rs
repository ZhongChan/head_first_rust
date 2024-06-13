use std::panic;
use head_first_rust::generate_main;
generate_main!(
    ("基本示例",basic)
);

/// # Rust 的错误哲学
/// 错误对于软件来说是不可避免的，因此一门优秀的编程语言必须有其完整的错误处理哲学。
/// 在很多情况下，`Rust` 需要你承认自己的代码可能会出错，并提前采取行动，来处理这些错误。
/// `Rust` 中的错误主要分为两类：
/// * *可恢复错误*，通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误，这些错误只会`影响某个用户自身的操作进程`，而不会对系统的全局稳定性产生影响。
/// * *不可恢复错误*，刚好相反，该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，这些错误的影响往往对于系统来说是致命的。
/// 很多编程语言，并不会区分这些错误，而是直接采用异常的方式去处理。
///
/// `Rust` 没有异常，但是 `Rust` 也有自己的卧龙凤雏：
/// * `Result<T, E>` 用于可恢复错误，
/// * `panic!` 用于不可恢复错误。
fn basic() {
    let result = panic::catch_unwind(|| {
        let v = vec![1, 2, 3];
        v[99];
    });

    match result {
        Ok(_) => {}
        Err(err) => {
            if let Some(s) = err.downcast_ref::<&str>() {
                println!("Panic caught: {:?}", s)
            } else if let Some(s) = err.downcast_ref::<String>() {
                println!("Panic caught: {:?}", s)
            } else {
                println!("Panic caught but could not determine the cause")
            }
        }
    }

    println!("Still running");
}


use std::net::IpAddr;

/// # 如何使用 `panic!`
/// ## 如下代码：
/// * `parse` 方法试图将字符串 `"127.0.0.1"` 解析为一个 IP 地址类型 `IpAddr`。
/// * 它返回一个 `Result<IpAddr, E>` 类型。
/// * 如果解析成功，则把 `Ok(IpAddr)` 中的值赋给 `home`，如果失败，则不处理 `Err(E)`，而是直接 `panic`。

fn handle_panic() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{:?}", home);
}