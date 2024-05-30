use crate::Action::ChangeColorRGB;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("match 匹配", Box::new(|| match_demo())),
        ("match 表达式赋值", Box::new(|| match_expression())),
        ("match 模式绑定", Box::new(|| match_binding())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
}

/// match 中用三个匹配分支来完全覆盖枚举变量 Direction 的所有成员类型，有以下几点值得注意：
/// * match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
/// * match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
/// * X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
fn basic() {
    let dire = Direction::West;
    match dire {
        Direction::East => { println!("East"); }
        Direction::West | Direction::North => {
            println!("West or North");
        }
        _ => { println!("West"); }
    }
}

/// match 匹配
/// # Example
/// ```
/// match target {
///     模式1 => 表达式1,
///     模式2 => {
///         语句1;
///         语句2;
///         表达式2
///     },
///     _ => 表达式3
/// }
/// ```
fn match_demo() {
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Penney));
    // 模式绑定
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penney,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penney => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => { 5 }
        Coin::Dime => { 10 }
        // state 变量将被绑定 UsState::Alaska 的枚举值
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[allow(dead_code)]
enum IpAddr {
    IpV4,
    IpV6,
}

fn match_expression() {
    let ip1 = IpAddr::IpV4;
    let ip_str = match ip1 {
        IpAddr::IpV4 => { "127.0.0.1" }
        _ => { "::1" }
    };
    println!("{}", ip_str);
}

#[allow(dead_code)]
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn match_binding() {
    let actions = [
        Action::Say("Hello Rust match".to_string()),
        Action::MoveTo(22, 23),
        ChangeColorRGB(255, 255, 0)
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0,0) move to ({},{})", x, y);
            }
            // _ 忽略参数
            ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{},g:{},b:0)','b' has been ignored", r, g);
            }
        }
    }
}