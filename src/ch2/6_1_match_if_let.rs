fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

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