fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("关联类型", Box::new(|| basic())),
        ("存储不同类型元素", Box::new(|| diff_element())),
        ("常用方法", Box::new(|| common_usage())),
        ("排序", Box::new(|| vec_sort())),
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

/// # Vector 排序
fn vec_sort() {
    i32_sort();
    f32_no_nan();
    f32_nan();
    unstable_stable();
    struct_sort();
    struct_default_sort();
}

/// # 整数数组排序
fn i32_sort() {
    let mut i32_v = vec![1, 7, 9, 2, 11];
    i32_v.sort_unstable(); //非稳定排序，比 稳定排序速度快 空间少
    assert_eq!(i32_v, [1, 2, 7, 9, 11]);
}

/// # 浮点数 无 NAN 排序
fn f32_no_nan() {
    let mut f32_v = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // f32_v.sort_unstable(); //Trait `Ord` is not implemented for `f32` [E0277] 浮点数有 NAN
    f32_v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); //我们确定在我们的浮点数数组当中，不包含 NAN 值，那么我们可以使用 partial_cmp 来作为大小判断的依据。
    assert_eq!(f32_v, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}

/// # 浮点数 有 NAN 排序
/// 自定义排序
fn f32_nan() {
    let mut nan_v = vec![1.0, 5.6, 10.3, 2.0, 15.0, f32::NAN, 3.3, f32::NAN];
    nan_v.sort_unstable_by(|a, b| {
        match (a.is_nan(), b.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => a.partial_cmp(b).unwrap(),
        }
    });
    println!("{:?}", nan_v); //NAN 通常在开头或末尾
}

/// # 稳定排序和非稳定排序
fn unstable_stable() {
    let items = vec![
        Item { value: 5, original_index: 1 },
        Item { value: 3, original_index: 2 },
        Item { value: 3, original_index: 3 },
        Item { value: 8, original_index: 4 },
        Item { value: 5, original_index: 5 },
    ];

    // 使用 sort_unstable
    let mut items_unstable = items.clone();
    items_unstable.sort_unstable_by(|a, b| a.value.cmp(&b.value));
    println!("After sort_unstable: {:?}", items_unstable);

    // 使用 sort_stable
    let mut items_stable = items.clone();
    items_stable.sort_by(|a, b| a.value.cmp(&b.value));
    println!("After sort_stable: {:?}", items_stable);
}

/// # 结构体排序
fn struct_sort() {
    let mut people = vec![
        Person::new("Foo".to_string(), 32),
        Person::new("FooBar".to_string(), 19),
        Person::new("AFoo".to_string(), 45),
    ];
    people.sort_unstable_by(|a, b| a.age.cmp(&b.age));
    println!("{:?}", people)
}

/// 结构体使用默认 Trait 排序
fn struct_default_sort() {
    let mut animals = vec![
        Animal::new("Dog".to_string(), 1),
        Animal::new("Snake".to_string(), 3),
        Animal::new("Cat".to_string(), 2),
    ];
    // 根据结构体定义字段的顺序，选择排序字段
    // 目前是 age
    animals.sort_unstable();
    println!("{:?}", animals);
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
struct Item {
    value: i32,
    original_index: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Animal {
    age: i32,
    category: String,
}

impl Animal {
    pub fn new(category: String, age: i32) -> Self {
        Self { category, age }
    }
}
