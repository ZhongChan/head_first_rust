fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("所有权", Box::new(|| owner_ship())),
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

/// self 依然有所有权的概念：
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