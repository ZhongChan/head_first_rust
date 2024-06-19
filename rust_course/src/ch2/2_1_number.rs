use num::complex::Complex;
use rust_course::generate_main;

generate_main!(
    ("整型溢出", u8_overflow),
    ("浮点数", float),
    ("NaN", number_nan),
    ("四则运算", four_operations),
    ("位运算", bit_operation),
    ("序列（Range）", range),
    ("有理数和复数", rational_and_complex_numbers)
);


/// # 整数溢出
/// * `wrapping_*` 方法：补码循环溢出
/// * `checked_*` 方法: `None`
/// * `overflowing_*` 方法：是否溢出标志位
/// * `saturating_*` 方法：计算结果在指定范围内 `[min, max]`

/// # `debug`: 报错
/// * `release`: 循环处理 `256->0` `257->1`。程序是错的

fn u8_overflow() {
    assert_eq!(255u8.wrapping_add(20), 19);

    assert_eq!(255u8.checked_add(20), None);
    assert_eq!(100u8.checked_add(20), Some(120));

    assert_eq!(255u8.overflowing_add(20), (19, true));
    assert_eq!(100u8.overflowing_add(20), (120, false));

    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
}

fn float() {
    let x = 2.0;//f64
    let y: f32 = 3.0;
    let z = 2;
    println!("{:.2},{:.2},{:.2}", x, y, z); //保留2位小数
    //2.00,3.00,2

    //IEEE 754 浮点数加法
    //f64 精度高
    assert_ne!(0.1 + 0.2, 0.3); //不相等
    println!("{}", 0.1 + 0.2); //0.30000000000000004
    println!("0.1 + 0.2 {} 0.3", f64_eq(0.1 + 0.2, 0.3));
    println!("0.3 + 0.2 {} 0.6", f64_eq(0.3 + 0.2, 0.6));

    //f32 精度低
    assert_eq!(0.1f32 + 0.2f32, 0.3f32); //true

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", abc.2.to_bits());

    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", xyz.2.to_bits());

    assert_eq!(abc.0 + abc.1, abc.2);
    // assert_eq!(xyz.0 + xyz.1, xyz.2); assertion `left == right` failed
}

// 使用足够小的公差判断浮点数是否相等
fn f64_eq(a: f64, b: f64) -> &'static str {
    let eps = 1e-10;//足够小的公差
    return if (a - b).abs() < eps {
        "="
    } else {
        "<>"
    };
}

// NaN
fn number_nan() {
    let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);
    //assertion `left == right` failed
    //   left: NaN
    //  right: NaN
    if x.is_nan() {
        println!("未定义的数学行为");
    }
}

/// # 四则运算
#[warn(clippy::inconsistent_digit_grouping)]
fn four_operations() {
    //基础示例 ： 基本类型一致才能运算
    println!("{}", 5 + 10);
    println!("{}", 95.5 - 10.1);
    println!("{}", 4 * 30);
    println!("{:.2}", 56.7 / 32.2);
    println!("{}", 43.2 % 10.3);

    //强制类型转换
    let a: i32 = 10;
    let b: f32 = 20.5;
    // let result = a: f32 * b; 语法错误
    let result = a as f32 * b;
    println!("10 * 20.5 = {}", result);

    //综合示例
    //编译器自动推导 i32
    let twenty = 20;
    //类型标注
    let twenty_one: i32 = 21;
    //类型后缀标注
    let twenty_two = 22i32;
    //同类型才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    //数字可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    //数组类型推导
    let forty_two = [
        42.0,
        42f32,
        42.0_32,
    ];
    //打印数组元素，保留两位小数
    println!("{:.2}", forty_two[0]);
}

//位运算
fn bit_operation() {
    let a: i32 = 2;
    let b: i32 = 3;
    println!("{:08b}", i32::MAX);
    //按最少8位展示二进制
    println!("{:08b}", a);
    println!("{:08b}", b);

    println!("(a & b) value is {} {:08b}", a & b, a & b);
    println!("(a | b) value is {} {:08b}", a | b, a | b);
    println!("(a ^ b) value is {} {:08b}", a ^ b, a ^ b);
    println!("(!b) value is {} {:08b}", !b, !b);
    println!("(a << b) value is {} {:08b}", a << b, a << b);
    println!("(a >> b) value is {} {:08b}", a >> b, a >> b);

    //可变量进行赋值
    let mut a = a;
    a <<= b;
    println!("(a << b) value is {} {:08b}", a, a);
}

//序列（Range）
//序列只允许使用数字或字符
fn range() {
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        print!("{}", i);
    }
    println!();
}

//有理数和复数
fn rational_and_complex_numbers() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}
