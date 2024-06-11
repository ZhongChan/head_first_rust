fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("关联类型", Box::new(|| basic())),
        ("存储不同类型元素", Box::new(|| diff_element())),
        ("常用方法", Box::new(|| common_usage())),
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

/// # Vector 常用方法
fn common_usage() {
    //初始化
    let v1 = vec![0; 3];
    let v2 = Vec::from([0, 0, 0]);
    assert_eq!(v1, v2);

    //扩容 减少内存拷贝
    let mut v3 = Vec::with_capacity(10);
    v3.extend([1, 2, 3]);
    println!("v3的长度是：{}，容量是：{}", v3.len(), v3.capacity());

    v3.reserve(100); //预留空间
    println!("v3 扩容后的长度是：{}，容量是：{}", v3.len(), v3.capacity());

    v3.shrink_to_fit();
    println!("v3 是否容量后的长度是：{}，容量是：{}", v3.len(), v3.capacity());

    //常用方法
    let mut v4 = vec![1, 2];
    assert!(!v4.is_empty());

    v4.insert(2, 5); // 在指定索引插入数据，索引值不能大于 v4 的长度， v: [1, 2, 5]
    assert_eq!(v4.remove(1), 2); //删除指定索引元素，返回值
    assert_eq!(v4.pop(), Some(5));  //弹出尾部元素
    assert_eq!(v4.pop(), Some(1)); //数组为空
    assert!(v4.is_empty());
    assert_eq!(v4.pop(), None);
    v4.clear();

    let mut v5 = [11, 3, 22].to_vec();
    v4.append(&mut v5); //v5所有元素添加到v4
    assert_eq!(v4, [11, 3, 22]);
    assert_eq!(v5, []);
    v4.truncate(2); //截取到指定长度
    assert_eq!(v4, [11, 3]);
    v4.retain(|x| *x > 10); //保留满足条件的元素
    assert_eq!(v4, [11]);

    let mut v6 = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同事获取被删除元素的迭代器：v6:[11,55] m:[22,33,44]
    let mut m: Vec<_> = v6.drain(1..=3).collect();
    assert_eq!(v6, [11, 55]);
    assert_eq!(m, [22, 33, 44]);

    //指定索引位置分割数组
    let v7 = m.split_off(1);
    assert_eq!(m, [22]);
    assert_eq!(v7, [33, 44]);

    //数组切片
    let v8 = vec![11, 22, 33, 44, 55];
    let v8_slice = &v8[1..=3]; //从 index 取 size 个元素
    assert_eq!(v8_slice, [22, 33, 44]);
}
