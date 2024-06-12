pub fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基础操作", Box::new(|| basic())),
        ("结构体内存", Box::new(|| struct_mem_sort())),
        ("元组结构", Box::new(|| tuple_struct())),
        ("单元结构体", Box::new(|| unit_like_struct())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: i64,
}

fn basic() {
    let u1 = User {
        active: false,
        username: "whom".to_string(),
        email: "foo@bar.com".to_string(),
        sign_in_count: 10,
    };
    println!("{:?}", u1);

    // 不支持某个字段单独标记为可变
    let mut u2 = User {
        active: false,
        username: "".to_string(),
        email: "".to_string(),
        sign_in_count: 2,
    };

    u2.active = true;
    u2.username = "foo_foo".to_string();
    u2.email = "foo_foo@qq.com".to_string();
    dbg!(u2);

    let u3 = build_user("foo@qq.com".to_string(), "foo_bar_bar".to_string());
    dbg!(u3);

    // 结构体更新语法
    let u4 = User {
        active: u1.active,
        username: u1.username,
        email: "strut_update@qq.com".to_string(),
        sign_in_count: 0,
    };
    dbg!(u4);

    let u5 = User {
        email: "foo@bar.com".to_string(),
        ..build_user("my@qq.com".to_string(), "my_name".to_string()) // 自动赋值只能放最后
    };
    dbg!(u5);

    let user1 = User {
        username: "foo".to_string(),
        email: "user1@qq.com".to_string(),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        username: user1.username, // 所有权转移
        email: "user2@qq.com".to_string(),
        active: user1.active, // 基本类型 copy
        sign_in_count: user1.sign_in_count, // 基本类型 copy
    };
    println!("{}", user1.active); // 正常打印
    //println!("{}", user1.username) // borrow of moved value: `user1.username`
    println!("{:?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: false,
        username, // 同名直接省略key
        email, // 同名直接省略key
        sign_in_count: 999,
    }
}


#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

/// # 结构体内存排序
fn struct_mem_sort() {
    let f1 = File { name: "f1.txt".to_string(), data: Vec::new() };
    let f_name = &f1.name;
    let f_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f_name, f_length);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Color(i32, i32, i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Point(i32, i32, i32);

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black);
    println!("{:?}", origin);
}


#[derive(Debug)]
struct AlwaysEqual;

fn unit_like_struct() {
    let subject = AlwaysEqual;
    println!("{:?}", subject);
}


#[allow(dead_code)]
struct UserLifetimes {
    // username: &str, //Missing lifetime specifier [E0106] todo 引入生命周期解决
    // email: &str, //Missing lifetime specifier [E0106] todo 引入生命周期解决
    sign_in_count: u64,
    active: bool,
}