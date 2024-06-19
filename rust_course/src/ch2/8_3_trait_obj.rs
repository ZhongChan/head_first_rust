use rust_course::generate_main;

generate_main!(
    ("使用枚举解决多结构体返回", enum_return_struct),
    ("特征对象的定义", trait_obj_def),
    ("self 和 Self", self_and_big_self),
    ("特征对象使用条件", object_safety)
);

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

/// 若 `T` 实现了 `BasicDraw` 特征，
/// 则调用该函数时传入的 `Box<T>` 可以被隐式转换成函数参数签名中的 `Box<dyn BasicDraw>`
/// `draw1` 函数的参数是 `Box<dyn Draw>` 形式的特征对象，该特征对象是通过 `Box::new(x)` 的方式创建的
/// `dyn` 关键字只用在特征对象的类型声明上，在创建时无需使用 `dyn`
fn draw1(x: Box<dyn BasicDraw>) {
    // 由于实现了 Deref 特征，
    // Box 智能指针会自动解引用为它所包裹的值，
    // 然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

/// `draw2` 函数的参数是 `&dyn Draw` 形式的特征对象，该特征对象是通过 `&x` 的方式创建的
/// `dyn` 关键字只用在特征对象的类型声明上，在创建时无需使用 `dyn`
fn draw2(x: &dyn BasicDraw) {
    x.draw();
}


/// # 特征对象的定义
/// # `Example`
/// ```
/// fn draw3(x: dyn Draw) {
///     x.draw();
/// }
/// ```
/// 在 `Rust` 中，尝试使用 `dyn Draw` 作为函数参数时，
/// `dyn Draw` 是一个特征对象，它本身是不定大小的（`unsized`）。
/// `Rust` 需要知道一个类型的确切大小来处理它，但是特征对象因为包含任意类型的数据，所以它们没有固定的大小。
/// 因此，不能直接将 `dyn Draw` 作为函数参数类型。
///
/// 为了解决这个问题，需要使用某种形式的指针来包装 `dyn Draw`。
/// 最常见的是使用引用（如 `&dyn Draw`）或者一个智能指针（如 `Box<dyn Draw>`）。
/// 这些指针类型是有大小的，因为它们本质上存储的是内存地址，而内存地址的大小是固定的。
///
/// ## 使用引用和 `Box` 的选择
/// * 使用 引用 (`&dyn Draw`) 是最简单和性能最高的方式，适用于临时借用一个实现了特定特征的对象。
/// * 使用 `Box` (`Box<dyn Draw>`) 提供了所有权和灵活性，适用于需要转移所有权或者在堆上存储数据的场景。
///
/// 根据具体需求选择合适的方式。
/// 在不需要拥有所有权的情况下，通常推荐使用引用，因为这样可以避免不必要的内存分配和性能开销。
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

#[allow(dead_code)]
trait Appearance {
    fn skin(&self) -> Self;
}

#[derive(Clone)]
#[derive(Debug)]
struct Duck;

impl Appearance for Duck {
    fn skin(&self) -> Self {
        self.clone()
    }
}

/// # `self`
/// * `self` 指向具体的对象实例。
/// * 在 `Rust` 中，`self` 用于实例方法中，代表方法被调用的具体实例。
/// * 根据方法的定义，`self` 可以是所有权形式 (`self`)，不可变引用 (`&self`)，或可变引用 (`&mut self`)。
/// * 这允许方法根据需要读取或修改实例的状态。
///
/// # `Self`
/// * `Self` 是类型本身的一个别名，确实可以视作一种语法糖，使得在特征或类型的实现中不必重复类型名称。
/// * 这在特征定义和实现时特别有用，因为它允许代码更加通用和易于维护。
/// * 当在类型 `T` 的 `impl` 块中使用 `Self` 时，`Self` 就代表了 `T`。
fn self_and_big_self() {
    let duck = Duck;
    let new_duck = duck.skin();
    println!("{:?}", new_duck);
}


/// ## 特征对象的安全
/// * 特质中的所有方法都必须至少有一个参数是 `self`（可以是 `&self` 或 `&mut self`）。
/// * 特质中不能包含任何返回 `Self` 的方法。
/// * 特质中不能使用泛型参数。
///
/// ## 特征对象的使用
/// * 特质对象通常通过指针（如 `Box<dyn Trait>` 或 `&dyn Trait`）来使用。
/// *
///
/// ## 为什么返回 `Self` 会破坏对象安全？
/// * 在使用特质对象时，具体的类型在编译时是未知的，只有在运行时才确定。
/// * 如果一个方法返回 `Self` 类型，编译器无法知道具体应该构造和返回哪种类型的实例，
/// * 因为 `Self` 的具体类型在使用特质对象时是不确定的。
///
/// 这就是为什么返回 `Self` 的方法会导致特质不是对象安全的。
///
/// ## 关联函数和对象安全
fn object_safety() {
    let dog = Dog;
    let cat = Cat;
    make_some_nosie(&dog);
    make_some_nosie(&cat);

    // let obj: Box<dyn MyTrait> = Box::new(5);  // 这行代码会报错，不满足特征对象安全
    let x = i32::new();
    println!("x: {}", x);
    println!("method: {}", x.method());
}


trait Speak {
    fn speak(&self);
}

struct Dog;

struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Miaoo!");
    }
}

fn make_some_nosie(animal: &dyn Speak) {
    animal.speak();
}

trait MyTrait {
    fn new() -> Self;
    fn method(&self) -> i32;
}

impl MyTrait for i32 {
    fn new() -> Self {
        0
    }

    fn method(&self) -> i32 {
        *self
    }
}