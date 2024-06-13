use std::{ptr, result};

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("悬垂引用和生命周期", Box::new(|| dangle_ref_lifetime())),
        ("函数中的生命周期", Box::new(|| function_lifetime())),
        ("生命周期标注语法", Box::new(|| lifetime_tag())),
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
/// todo 优化文档引用
fn dangle_ref_lifetime() {
    let r;
    {
        let x = 5;
        r = &x; //error[E0597]: `x` does not live long enough
        println!("{}", r)
    }
    // println!("{}", r)
}

fn function_lifetime() {
    let s1 = String::from("Lifetime");
    let s2 = "Rust";
    // let result = longest_dangle(s1.as_str(), s2); //Process finished with exit code 139 (interrupted by signal 11:SIGSEGV) 访问被释放的内存
    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);
}

/// # 生命周期
/// 生命周期参数 'a：
/// * 我们在 longest 函数中添加了生命周期参数 'a，
/// * 这表示 s1 和 s2 的引用必须在同一个生命周期内，
/// * 并且返回的引用也将具有相同的生命周期。
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/// # 使用 unsafe 破坏内存
/// ## 解释
/// 1. Box::new：我们使用 Box 来分配在堆上的字符串，这样我们可以手动释放它。
/// 2. unsafe 块：使用 unsafe 块绕过 Rust 的借用检查器。
/// 3. 指针转换：将 s1 转换为一个原始指针 *const str。
/// 4. 手动释放：通过 ptr::drop_in_place 手动释放 s1 所指向的 Box<String>，这会使 s1 成为悬垂引用。
/// 5. 悬垂引用：尝试通过原始指针 s1_ptr 访问已经被释放的内存。
#[allow(dead_code)]
fn longest_dangle<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    unsafe {
        // 将 s1 转换为一个原始指针
        let s1_ptr = s1 as *const str;

        // 手动释放 s1 所指向的 Box<String>
        let s1_boxed = s1_ptr as *const Box<String>;
        ptr::drop_in_place(s1_boxed as *mut Box<String>);

        // 尝试通过原始指针访问已经被释放的内存
        if (*s1_ptr).len() > s2.len() {
            &*s1_ptr
        } else {
            s2
        }
    }
}

/// # 生命周期标注语法
/// `生命周期标注并不会改变任何引用的实际作用域`
///
fn lifetime_tag() {
    lifetime_the_little();
}

/// ## 返回参会生命周期等于请求参数中较小那个
fn lifetime_the_little() {
    let long = "long string is long".to_string();
    {
        let short = "short".to_string();
        let result = longest(long.as_str(), short.as_str());
        println!("The longest string is: {}", result);
    }
}

/// ## 返回参会生命周期等于请求参数中较小那个
/// ## 错误示例
/// ```rust
/// fn lifetime_the_little_wrong() {
///     let long = "long string is long".to_string();
///     let result;
///     {
///         let short = "short".to_string();
///         result = longest(long.as_str(), short.as_str());
///     } //     `short` dropped here while still borrowed
///     println!("The longest string is: {}", result);
/// }
/// ```
///
/// ## 编译时报错：
/// ```text
///error[E0597]: `short` does not live long enough
///    --> src/ch2/10_0_lifetime.rs:156:41
///     |
/// 155 |         let short = "short".to_string();
///     |             ----- binding `short` declared here
/// 156 |         result = longest(long.as_str(), short.as_str());
///     |                                         ^^^^^ borrowed value does not live long enough
/// 157 |     } //     `short` dropped here while still borrowed
///     |     - `short` dropped here while still borrowed
/// 158 |     println!("The longest string is: {}", result);
///     |                                           ------ borrow later used here
/// ```
///
/// ## 错误原因
/// 1. *生命周期参数* `'a：longest` 函数的生命周期参数 `'a` 表示返回值的生命周期必须与两个输入引用的生命周期中较短的那个相同。
/// 2. *编译器无法判断*：在 `longest` 函数内部，编译器无法判断 `result` 引用的是 `long` 还是 `short`。因此，为了安全起见，编译器假设 `result` 的生命周期与 `short` 一样长。
/// 3. *块作用域*：`short` 在块结束时被释放，而 `result` 可能引用了 `short`，这就导致了潜在的悬垂引用。
///
///
fn lifetime_the_little_wrong() {
    let long = "long string is long".to_string();
    let result;
    {
        let short = "short".to_string();
        result = longest(long.as_str(), short.as_str());
        println!("The longest string is: {}", result);
    }
}
