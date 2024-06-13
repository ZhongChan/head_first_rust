use head_first_rust::generate_main;

generate_main!(
    ("基本示例", basic),
    ("while let", while_let),
    ("for 循环", for_loop),
    ("let 语句", let_expression),
    ("函数参数", func_args)
);


/// # 模式
/// 模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据，它往往和 match 表达式联用，以实现强大的模式匹配能力。模式一般由以下内容组合而成：
///
/// * 字面值
/// * 解构的数组、枚举、结构体或者元组
/// * 变量
/// * 通配符
/// * 占位符
/// # 所有可能用到模式的地方
///
/// ## match 分支
///```
/// match VALUE {
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
/// }
///```
/// 如上所示，match 的每个分支就是一个模式，因为 match 匹配是穷尽式的，因此我们往往需要一个特殊的模式 _，来匹配剩余的所有情况：
/// ```
/// match VALUE {
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
///     _ => EXPRESSION,
/// }
/// ```
/// ## if let 分支
/// if let 往往用于匹配一个模式，而忽略剩下的所有模式的场景：
/// ```
/// if let PATTERN = SOME_VALUE {
///
/// }
/// ```
///
///
///
///
fn basic() {}


fn while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop() 返回 Option<T>
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}


/// # for 循环
/// 这里使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配。
fn for_loop() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

/// # let 语句
/// ```let PATTERN = EXPRESSION;```
fn let_expression() {
    let (x, y, z) = (1, 2.0, 'c');
    dbg!((x, y, z));

    // let (r,g,b) = (1,2); //Type mismatch [E0308]
    // let (r, g) = (1, 2, 3); //Type mismatch [E0308]
}

/// 函数参数
/// ```
/// fn foo(x: i32) {
///     // 代码
/// }
/// ```
/// 其中 x 就是一个模式，你还可以在参数中匹配元组：
///
fn func_args() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({},{})", x, y);
}
