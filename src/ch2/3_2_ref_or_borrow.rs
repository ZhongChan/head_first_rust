use head_first_rust::generate_main;

generate_main!(
    ("基本示例", basic),
    ("不可变引用", not_mut_ref),
    ("可变引用", mut_ref),
    ("可变引用同时只有有一个", only_one_mut_ref),
    ("NLL", nll),
    ("悬垂引用", dangle_ref)
);

fn basic() {
    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    // assert_eq!(y, 5);// can't compare `&{integer}` with `{integer}`
    assert_eq!(*y, 5);//解引用，获取值
}

fn not_mut_ref() {
    let s1 = String::from("hello");
    let l = cal_len(&s1);
    println!("{} length is {}", s1, l)
}

fn cal_len(some_string: &str) -> usize {
    some_string.len()
}

fn mut_ref() {
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

fn only_one_mut_ref() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s3 = &mut s1;
    // println!("s1:{},s2:{},s3:{}", s1, s2, s3); // cannot borrow `s1` as mutable more than once at a time
    println!("s2:{}", s2);

    let mut t1 = String::from("world");
    {
        let t2 = &mut t1;
        // println!("t1:{}", t1); // cannot borrow `t1` as immutable because it is also borrowed as mutable
        println!("t2:{}", t2);
    }
    let t3 = &mut t1;
    println!("t3:{}", t3);
}

/// # None-Lexical Lifetimes
/// * 在 Rust 2015 edition 中，
/// * 变量的生命周期是以词法（`lexical`）的方式确定的，
/// * 它是由变量在源代码中出现的位置来决定的。
/// * 这种方式有时候会过于保守，导致一些本应该被允许的代码被错误地拒绝。
///
///  # Examples
///
///  ```
///     fn main() {
///         let mut x = 5;
///         let y = &x;
///         x += 1; // `Error` in Rust 2015
///         println!("{}", y);
///     }
///  ```
/// * 在 Rust 2015 中，这段代码会报错，因为 `y` 的生命周期从它被创建开始，
/// * 一直延续到它最后一次被使用的地方结束。
/// * 因此，`x += 1;` 这行代码尝试修改 `x` 的值，
/// * 但是 `x` 在这个时间点上被 `y` 借用，所以这是不允许的。
///
/// * 然而，从逻辑上讲，`y` 在 `x += 1;` 这行代码执行之前并没有被实际使用，
/// * 所以应该允许修改 `x` 的值。这就是 `NLL` 能够处理的情况。
/// * 在 Rust 2018 中，上述代码是被允许的，因为借用检查器会识别到 `y` 在 `x += 1;` 执行之前并没有被实际使用。
/// * 通过 `NLL`，Rust 的借用检查器能够更准确地理解变量的生命周期，使得 Rust 的借用规则更加灵活和实用。
fn nll() {
    let mut s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    //s2 s3 是多个不可变引用，且生命周期结束(最后一次使用)
    //println 只是对 s2 s3的借用
    println!("s2:{},s3:{}", s2, s3);


    let s4 = &mut s1; //只能由一个可变引用
    println!("s4:{}", s4);
}

/// # 悬垂引用
/// # Examples
///
/// ```
///  fn dangle() -> &String { //返回字符串的引用
///     let s = String::from("hello");
///     &s //返回字符串 `s` 的引用
///  } // `s` 内存被释放
/// ```
/// * 因为 `s` 是在 `dangle` 函数内创建的，当 `dangle` 的代码执行完毕后，`s` 将被释放，
/// * 但是此时我们又尝试去返回它的引用。
/// * 这意味着这个引用会指向一个无效的 `String`，这可不对！
pub fn dangle_ref() {
    println!("{}", no_dangle());
}

fn no_dangle() -> String {
    String::from("hello")
}
