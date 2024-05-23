#![allow(unused_variables)]
#[macro_use]
extern crate head_first_rust;

/// 从代码设计角度来看，关于文件操作的类型和函数应该组织在一起，
/// 散落得到处都是，是难以管理和使用的。
///
/// 而且通过 open(&mut f1) 进行调用，
/// 也远没有使用 f1.open() 来调用好，
/// 这就体现出了只使用基本类型的局限性：无法从更高的抽象层次去简化代码。
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

/// 发散函数
/// unimplemented!() 告诉编译器该函数尚未实现，
/// unimplemented!() 标记通常意味着我们期望快速完成主要代码，
/// 回头再通过搜索这些标记来完成次要代码，类似的标记还有 todo!()，
/// 当代码执行到这种未实现的地方时，程序会直接报错。
/// 你可以反注释 read(&mut f1, &mut vec![]); 这行，然后再观察下结果。
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("切片", Box::new(|| slice())),
        ("字符串基础", Box::new(|| string_basic())),
        ("String与&str的转换", Box::new(|| string_str())),
        ("字符串索引", Box::new(|| string_index())),
        ("字符串切片", Box::new(|| string_slice())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}


fn basic() {
    let mut f = File::from("file1.txt");
    open(&mut f);
    // read(&mut f, &mut vec![]);
    close(&mut f);
}

fn slice() {
    let s = String::from("hello,rust");
    let s1 = &s[0..4]; //s的不可变引用
    let s2 = &s[3..7]; //s的不可变引用
    println!("s1:{},s2:{}", s1, s2, );

    //最后一个字节
    let s3 = &s[4..s.len()];
    let s4 = &s[4..];
    println!("s3:{},s4:{}", s3, s4);

    //整个字符串
    let s5 = &s[0..s.len()];
    let s6 = &s[..];
    println!("s5:{},s6:{}", s5, s6);

    let utf8_str = String::from("中国人");//每个汉字3个字节
    // let u1 = &utf8_str[0..2]; 报错
    let u1 = &utf8_str[0..3]; //打印：中
    println!("u1:{}", u1);

    //消除编译器警告，示例用
    #[allow(warnings)] let mut all = String::from("hello,everyone");
    let f = first_world(&all); //不可变引用
    // everyone.clear(); //可变引用  cannot borrow `everyone` as mutable because it is also borrowed as immutable
    println!("all:{},first_word:{}", all, f); //不可变引用
}

fn first_world(s: &String) -> &str {
    &s[..1]
}

/// 字符串基础
/// 字符 是Unicode 编码（4字节）
/// 字符串 utf-8 编码（1~4字节）
fn string_basic() {
    let s1 = "中";
    print_size_of_val!(s1);
    let c1 = '中';
    print_size_of_char!(c1);
}

/// String 与 &str 转换
/// &str 硬编码进可执行文件、utf-8编码
/// String 可变长度、utf-8编码、所有权
fn string_str() {
    let s = String::from("hello,rust");
    say_hello(&s); //不可变引用
    say_hello(&s[..]); //切片
    say_hello(s.as_str()); //底层直接返回 &str
}

fn say_hello(s: &str) {
    println!("{}", s);
}

/// String 和 &str 变长 ，按index取出来也没有意义
fn string_index() {
    let s = "你好，rust";//您好，9字节+4字节
    // let h = s[0]; //The type `str` cannot be indexed by `{integer}` [E0277]
    print_size_of_val!(s); //13字节
    println!("{:?}", s.as_bytes());
}

/// 危险：字符串切片
/// 字符串要去校验每个字符的边界
fn string_slice() {
    let s = "你好，rust";//您好，9字节+4字节
    println!("{}", s);
    // let h = &s[0..2]; //byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好，rust`
    let h: String = s.chars().take(3).collect();
    println!("{}", h);
}