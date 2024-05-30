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
    let c = Circle::new(10.0, 20.0, 5.0);
    println!("{}", c.area());

    let r = Rectangle::new(4f64, 9f64);
    println!("{}", r.area());
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

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