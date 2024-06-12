use std::collections::HashMap;

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