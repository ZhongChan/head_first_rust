fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("使用枚举解决多结构体返回", Box::new(|| enum_return_struct())),
        ("特征对象的定义", Box::new(|| trait_obj_def())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}


/// 使用条件编译，编码这个错误的函数被编译
/// 该函数返回了不同的 结构体
#[cfg(feature = "example")]
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Post {
            title: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Weibo {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
        }
    }
}

#[derive(Debug)]
enum UiObject {
    Button,
    SelectBox,
}

fn draw(o: UiObject) {
    println!("{:?}", o);
}

/// 在编写这个 UI 库时，我们无法知道所有的 UI 对象类型，只知道的是：
/// * UI 对象的类型不同
/// * 需要一个统一的类型来处理这些对象，无论是作为函数参数还是作为列表中的一员
/// * 需要对每一个对象调用 draw 方法
fn enum_return_struct() {
    let objects = [UiObject::Button, UiObject::SelectBox];
    for o in objects {
        draw(o);
    }
}

pub trait Draw {
    fn drawing(&self);
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn drawing(&self) {
        println!("{:?}", self);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn drawing(&self) {
        println!("{:?}", self);
    }
}


/// 动态数组存储 UI 对象
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for x in self.components.iter() {
            x.drawing();
        }
    }
}

trait BasicDraw {
    fn draw(&self) -> String;
}

impl BasicDraw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl BasicDraw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

/// 若 T 实现了 BasicDraw 特征，
/// 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn BasicDraw>
/// draw1 函数的参数是 Box<dyn Draw> 形式的特征对象，该特征对象是通过 Box::new(x) 的方式创建的
/// dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
fn draw1(x: Box<dyn BasicDraw>) {
    // 由于实现了 Deref 特征，
    // Box 智能指针会自动解引用为它所包裹的值，
    // 然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

/// draw2 函数的参数是 &dyn Draw 形式的特征对象，该特征对象是通过 &x 的方式创建的
/// dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
fn draw2(x: &dyn BasicDraw) {
    x.draw();
}

/// # 特征对象的定义
fn trait_obj_def() {
    let x = 1.1f64;
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，
    // 因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    draw1(Box::new(x)); //基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(y));//基于 y 的值创建一个 Box<u8> 类型的智能指针

    draw2(&x);
    draw2(&y);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "Yes".to_string(),
                    "MayBe".to_string(),
                    "No".to_string(),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".to_string(),
            }),
            Box::new("Hi".to_string()),
        ],
    };

    screen.run();
}

/// 为 String 实现 Draw 特征
impl Draw for String {
    fn drawing(&self) {
        println!("Draw String {:?}", self);
    }
}