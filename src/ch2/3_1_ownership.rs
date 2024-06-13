#[macro_use]
use head_first_rust::generate_main;
use head_first_rust::print_size_of_val;

generate_main!(
    ("所有权原则", ownership_principle),
    ("深拷贝和浅拷贝", clone_and_copy),
    ("函数值和返回", fn_args_and_ret)
);



/// 所有权原则
/// 1. Rust 中每一个值都被一个变量拥有，该变量被称为值的所有者
/// 2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
/// 3. 当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)
fn ownership_principle() {
    demo_of_string();
    borrow_of_moved_value();
    solve_borrow_of_moved_value();
    val_binding();
    box_basic_type();
}

fn demo_of_string() {
    // 字符串字面量
    // 不可变、静态分配
    let s = "Hello";
    println!("the length of s {}", s.len());

    // String 对象，不可变
    let s2 = String::from("hello");
    //s2.push_str(",world"); //Cannot borrow immutable local variable `s2` as mutable
    println!("the length of s2 {}", s2.len());

    // String 对象，可变
    let mut s3 = String::from("hello");
    s3.push_str(",world");
    print_size_of_val!(s3);
}

/// 变量绑定
fn val_binding() {
    let x = 5;
    let mut y = x;
    y = y + 1;
    println!("{}", x);
    println!("{}", y);
}

/// # 预防优于治疗
/// * Rust 没有类似于 Go 的 panic 和 recover 机制。
/// * Rust 的错误处理哲学是“预防优于治疗”，
/// * 它鼓励开发者通过编译时检查来预防错误，
/// * 而不是在运行时捕获它们。
/// * Rust 的错误处理主要依赖于返回 Result 或 Option 类型，
/// * 以及使用 match 语句或 if let 表达式来处理这些类型。
fn borrow_of_moved_value() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // borrow of moved value: `s1`
    println!("{}", s2);
}

/// # String 如何防止 borrow of moved
fn solve_borrow_of_moved_value() {
    // 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1:{},s2:{}", s1, s2);

    // & 借用：无使用权的情况下，使用 s3 的内容，借用 s3 的不可变引用。
    let s3 = String::from("hello");
    let s4 = &s1;
    println!("s3:{},s4:{}", s3, s4);
}

fn box_basic_type() {
    let x = Box::new(5);
    let y = x;
    //println!("{}", x);  // borrow of moved value: `x`
    println!("{}", y);
}

fn clone_and_copy() {
    // 深拷贝：栈指针和堆数据
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1:{},s2:{}", s1, s2);

    // 浅拷贝
    // 所有的基本类型 和 包含基本类型的 ()
    let i: i32 = 10;
    let j = i;
    println!("i:{},j:{}", i, j);

    let f1: f64 = 11.1;
    let f2 = f1;
    println!("f1:{},f2:{}", f1, f2);

    let t1 = true;
    let t2 = t1;
    println!("t1:{},t2:{}", t1, t2);

    let ut = (10, 12.0);
    let ut2 = ut;
    println!("ut:{:?},ut2:{:?}", ut, ut2);

    // 只能是深拷贝
    let nt = (1, String::from("hello"));
    let nt2 = nt.clone();
    println!("nt:{:?},nt2:{:?}", nt, nt2);
}

fn fn_args_and_ret() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); //borrow of moved value: `s`

    let x = 5;
    makes_copy(x);
    println!("x:{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);
    println!("s1:{},s3:{}", s1, s3); //s2 没有所有权
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}