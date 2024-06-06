fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![("关联类型", Box::new(|| basic()))];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn basic() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<f32> = Vec::new();
    dbg!(v2);
    let mut v3 = Vec::new();
    v3.push(1);
    dbg!(v3);

    {
        let v4 = vec!["hello", "rust"];
        dbg!(v4);
    }
    // println!("{}",v4);//cannot find value `v4` in this scope

    // 访问元素
    // let does_not_exits = &v1[100]; //index out of bounds: the len is 3 but the index is 100
    let default_value = v1.get(100);
    match default_value {
        Some(i) => println!("the value is {}", i),
        None => println!("nothing get"),
    }
}
