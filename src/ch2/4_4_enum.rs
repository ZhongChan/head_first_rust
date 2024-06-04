fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("枚举值", Box::new(|| enum_value())),
        ("Option 枚举值", Box::new(|| enum_option())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

/// 扑克的花色
#[derive(Debug)]
#[allow(dead_code)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
#[allow(dead_code)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

#[derive(Debug)]
#[allow(dead_code)]
enum PokerCardSuit {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

#[derive(Debug)]
#[allow(dead_code)]
enum PokerCartSuitOther {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit, // 无关联数据
    Move { x: i32, y: i32 }, // 匿名结构体
    Write(String), // 包含一个字符串
    ChangeColor(i32, i32, i32), // 包含多个 i32
}

fn enum_value() {
    let hearts = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(hearts);
    print_suit(diamond);

    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    println!("{:?}", c1);

    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 13,
    };
    println!("{:?}", c2);

    // 数据信息 关联到成员
    let c1_enum = PokerCardSuit::Clubs(1);
    let c2_enum = PokerCardSuit::Diamonds(13);
    println!("{:?}", c1_enum);
    println!("{:?}", c2_enum);

    // 不同的类型的值数据
    let d1_enum = PokerCartSuitOther::Clubs(1);
    let d2_enum = PokerCartSuitOther::Hearts('A');
    println!("{:?}", d1_enum);
    println!("{:?}", d2_enum);

    // 任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::ChangeColor(255, 255, 0);
    println!("{:?}", (m1, m2, m3));
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

/// Option 枚举处理空值
/// 尽管如此，空值的表达依然非常有意义，
/// 因为空值表示当前时刻变量的值是缺失的。
/// 有鉴于此，Rust 吸取了众多教训，
/// 决定抛弃 null，而改为使用 Option 枚举变量来表述这种结果。
/// Option 枚举包含两个成员，一个成员表示含有值：
/// Some(T), 另一个表示没有值：None，定义如下：
///
/// # Example
/// ```
/// enum Option<T> {
///     Some(T), // T 是泛型参数。Some 可以包含任意类型数据
///     None,
/// }
/// ```
///
///
fn enum_option() {
    let some_number = Some(5);
    dbg!(some_number.is_none());

    let some_string = Some("hello");
    dbg!(some_string.is_none());

    // 使用 None 需要告诉 Option 是什么类型
    let absent_number: Option<i32> = None;
    dbg!(absent_number.is_none());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
    dbg!(none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => { Some(i + 1) }
    }
}