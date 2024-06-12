use std::collections::HashMap;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("所有权转移", Box::new(|| owner_ship())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn basic() {
    // 基本创建
    let mut map = HashMap::new();
    map.insert("ruby", 1);
    map.insert("Emerald", 2);
    map.insert("Topaz", 5);
    println!("{:?}", map);


    // 数组转化
    let team_list = vec![
        ("中国队", 100),
        ("美国队", 99),
        ("日本队", 1),
    ];

    // 需要手动标注类型为：HashMap<_,_>
    let team_map: HashMap<_, _> = team_list.into_iter().collect();
    println!("{:?}", team_map)
}

/// # 所有权转移

fn owner_ship() {
    string_no_copy();
    ref_lifetime_with_map();
}

/// ```HashMap``` 的所有权规则与其它 ```Rust``` 类型没有区别：
/// * 若类型实现 ```Copy``` 特征，该类型会被复制进 ```HashMap```，因此无所谓所有权
/// * 若没实现 ```Copy``` 特征，所有权将被转移给 ```HashMap``` 中
fn string_no_copy() {
    let name = String::from("Zzz"); //move occurs because `name` has type `String`, which does not implement the `Copy` trait
    let age = 18;

    let mut person = HashMap::new();
    person.insert(name.clone(), age);
    // person.insert(name, age); //value moved here
    println!("{}", name); //value borrowed here after move
    println!("{}", age); // 基本类型，实现了 Copy ，只是借用
}

/// 使用引用类型放入 ```Map``` 需要确保引用类型生命周期比 ```Map``` 长
fn ref_lifetime_with_map() {
    let name = String::from("Zzz");
    let age = 18;

    let mut person = HashMap::new();
    person.insert(&name, age); // 使用引用

    // value borrowed here after move
    // drop(name); //内存回收

    println!("{}", name);
    println!("{}", age);
}