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
        ("字符串操作", Box::new(|| string_operating())),
        ("字符串转义", Box::new(|| string_escape())),
        ("字符串不转义", Box::new(|| string_unescape())),
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

fn string_operating() {
    /*
    追加 Push
    */
    let mut s1 = String::from("hello");
    //追加字符串
    s1.push_str(",rust");
    println!("{}", s1);
    //追加字符
    s1.push('!');
    println!("{}", s1);

    /*
    插入 Insert
    */
    s1.insert(5, '你');
    println!("{}", s1);

    s1.insert_str(8, "好啊"); //和字符串切片一样的问题，utf-8变长编码，位置不一定对
    println!("{}", s1);

    /*
    替换 Replace
     */
    let new_s1 = s1.replace("rust", "RUST");
    println!("{}", new_s1);
    // 如果没有匹配到 直接返回原字符串切片
    assert_eq!(s1, s1.replace("foo", "bar"));

    let s2 = "foo foo 123 foo";
    assert_eq!("new new 123 foo", s2.replacen("foo", "new", 2)); //只匹配2次
    assert_eq!("faa fao 123 foo", s2.replacen('o', "a", 3)); //只匹配3才
    assert_eq!("foo foo newnew3 foo", s2.replacen(char::is_numeric, "new", 2)); //数字 替换成 new 匹配2次

    // 如果没有匹配到 直接返回原字符串切片
    assert_eq!(s2, s2.replacen("bar", "foo_bar", 1));

    //修改字符串
    let mut s3 = String::from("I like rust!");
    s3.replace_range(7..8, "R");
    dbg!(s3);

    /*
    删除 Delete
    */
    // pop 删除并返回字符最后一个字符
    let mut s4 = String::from("rust pop 中文！");
    let p1 = s4.pop();
    let p2 = s4.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(s4);

    // delete
    let mut s5 = String::from("测试remove方法");
    s5.remove(0);
    // s5.remove(1); //byte index 1 is not a char boundary 老问题了，字符串切片 utf-8编码
    dbg!(s5);

    // truncate
    let mut s6 = String::from("测试truncate");
    // s6.truncate(2); //assertion failed: self.is_char_boundary(new_len) 老问题了，字符串切片 utf-8编码
    s6.truncate(3);
    dbg!(s6);

    // clear
    let mut s7 = String::from("测试clear"); //6+5=11
    s7.clear();
    assert!(s7.is_empty());
    assert_eq!(0, s7.len());
    assert_eq!(11, s7.capacity());

    /*
    连接 Concatenate
    */
    let s8 = String::from("hello,");
    let s9 = String::from("rust");
    // s8自动解引用为 &str 本质是调用 add 方法
    // + 运算，左侧只能是 String
    // 根据 add 的定义，s8 的所有权被转移了
    let result = s8 + &s9; //value moved here
    // dbg!(s8);
    let mut result = dbg!(result) + "!"; //String + &str
    result += "!!!"; //使用的是 push_str()
    dbg!(result);

    // 使用 format
    let s10 = "hello,";
    let s11 = String::from("rust");
    let s12 = format!("{}{}!", s10, s11);
    println!("{}", s12);
}

fn string_escape() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    //使用 \ 忽略换行
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}

fn string_unescape() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}