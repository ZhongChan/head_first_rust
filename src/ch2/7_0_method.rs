pub fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("所有权", Box::new(|| owner_ship())),
        ("自动引用", Box::new(|| auto_referencing())),
        ("自动解引用", Box::new(|| auto_dereferencing())),
        ("enum 实现方法", Box::new(|| enum_impl())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn basic() {
    let c = Circle::new(10.0, 20.0, 5.0);
    println!("{}", c.area());

    let r = Rectangle::new(4f64, 9f64);
    println!("{}", r.area());
    println!("{}", r.perimeter());
    r.anther();
}

#[allow(dead_code)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[allow(dead_code)]
impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self { x, y, radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(self: &Self) -> f64 {
        (self.width + self.height) * 2f64
    }
}


/// # 多个 impl 方便文件分块
/// new 只能有一个
/// 关联函数、方法 都不支持重载
impl Rectangle {
    fn anther(&self) {
        println!("anther impl:({}, {})", self.width, self.height);
    }
}

/// # self 依然有所有权的概念：
/// * ```self``` 表示所有权转移到该方法中，这种形式用的较少
/// * ```&self``` 表示该方法对实例的不可变借用
/// * ```&mut self``` 表示可变借用
fn owner_ship() {
    let inst1 = Example::new("hello".to_string());
    inst1.consume();
    // println!("{}", inst1.data); //error[E0382]: borrow of moved value: `inst1`

    let inst2 = Example::new("Hi".to_string());
    inst2.read();
    println!("Still here: {}", inst2.data);

    let mut inst3 = Example::new("Hello".to_string());
    inst3.modified();
    println!("{}", inst3.data);
}

struct Example {
    data: String,
}

#[allow(dead_code)]
impl Example {
    //注意这里是 关联函数(静态方法) 没有 self
    pub fn new(data: String) -> Self {
        Self { data }
    }
    pub fn data(&self) -> &str {
        &self.data
    }
    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    fn consume(self) {
        println!("Consuming data {}", self.data);
    }

    fn read(&self) {
        println!("Reading data {}", self.data);
    }

    fn modified(&mut self) {
        self.data.push_str(",Rust")
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn get_x(&self) -> i32 {
        self.x
    }
}

/// # 自动引用 (Auto-Referencing)
/// 当你调用一个对象的方法时，Rust 会自动根据方法签名添加 & 或 &mut。
/// 这意味着即使你有一个值，而方法期望一个引用，你不需要显式地引用这个值；Rust 会为你处理这个引用。
fn auto_referencing() {
    let mut p = Point { x: 0, y: 0 };
    // 直接调用，无需显式使用 &mut point
    p.move_by(12, 33);
    println!("Point move to:({}, {})", p.x, p.y);
}

/// # 自动解引用 (Auto-Dereferencing)
/// 当你调用一个方法时，如果该方法是在一个引用的类型上定义的（如 Box, &, Rc, 等），
/// Rust 将自动解引用这个引用以匹配方法。这意味着你可以在一个类型的引用上调用直接定义在该类型上的方法。
fn auto_dereferencing() {
    let p = Point { x: 20, y: 30 };
    let p_ref = &p;
    // 手动解引用
    println!("The x coordinate is :{}", (*p_ref).get_x());
    // 自动解引用，无需写 (*p_ref).get_x()
    println!("The x coordinate is :{}", p_ref.get_x());
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {}
            Message::Move { .. } => {}
            Message::Write(msg) => {
                println!("{}", msg);
            }
            Message::ChangeColor(_, _, _) => {}
        }
    }
}

fn enum_impl() {
    let m = Message::Write("hello,enum impl".to_string());
    m.call();
}