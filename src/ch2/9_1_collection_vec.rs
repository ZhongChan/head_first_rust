fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("关联类型", Box::new(|| basic())),
        ("存储不同类型元素", Box::new(|| diff_element())),
    ];

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

    let v5 = vec![1, 2, 3];
    for ele in &v5 {
        println!("{ele}")
    }

    let mut v6 = vec![4, 5, 6];
    for ele in &mut v6 {
        *ele += 10;
    }
}

/// # 同时借用多个数组元素
/// 这段代码无法执行
/// 使用条件编译，编码这个错误的函数被编译
#[cfg(feature = "multi_borrow")]
fn multi_borrow() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow occurs here
    v.push(6); //mutable borrow occurs here
    println!("the first element is: {first}"); // immutable borrow later used here
}

/// 存储不能类型元素
fn diff_element() {
    let ips = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
    for x in ips {
        println!("{:?}", x);
    }


    let ip_struts: Vec<Box<dyn IpAddr2>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for x in ip_struts {
        x.display();
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}


trait IpAddr2 {
    fn display(&self);
}

struct V4(String);

impl IpAddr2 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);

impl IpAddr2 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}