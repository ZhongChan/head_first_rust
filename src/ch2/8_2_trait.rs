use std::fmt::{Debug, Display, Formatter};

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("孤儿规则", Box::new(|| orphan_rule())),
        ("使用特征作为函数参数", Box::new(|| trait_as_params())),
        ("特征约束", Box::new(|| trait_bound())),
        ("Where 约束", Box::new(|| where_bound())),
        ("有条件实现方法或特征", Box::new(|| condition_bound())),
        ("函数返回 Trait", Box::new(|| return_impl_trait())),
        ("newtype", Box::new(|| new_type())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn basic() {
    let post = Post::new("Head first Rust".to_string(), "Zhong".to_string(), "一本介绍Rust的书籍".to_string());
    println!("{}", post.summarize());
    println!("{}", post.something()); //默认实现
    let weibo = Weibo::new("重".to_string(), "我发了一条微博".to_string());
    println!("{}", weibo.summarize());
    println!("{}", weibo.something()); //覆盖默认实现
}

/// # 孤儿规则（Orphan Rule）
/// * 如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
fn orphan_rule() {
    let post = Post::new("Head first Rust".to_string(), "Zhong".to_string(), "一本介绍Rust的书籍".to_string());
    println!("{}", post);
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Say {
    fn something(&self) -> String {
        "Read more...".to_string()
    }
}

#[allow(dead_code)]
struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Post {
    pub fn new(title: String, author: String, content: String) -> Self {
        Self { title, author, content }
    }
}

impl Say for Post {}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章：{}，作者：{}", self.title, self.author)
    }
}

/// 为 Post 实现 Display 特征
impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nAuthor: {}\nContent: {}",
            self.title, self.author, self.content,
        )
    }
}

#[allow(dead_code)]
struct Weibo {
    pub username: String,
    pub content: String,
}

impl Weibo {
    pub fn new(username: String, content: String) -> Self {
        Self { username, content }
    }
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} 发表了微博=> {}", self.username, self.content)
    }
}

impl Say for Weibo {
    fn something(&self) -> String {
        "微博".to_string()
    }
}

fn trait_as_params() {
    let post = Post::new("Head first Rust".to_string(), "Zhong".to_string(), "一本介绍Rust的书籍".to_string());
    let weibo = Weibo::new("重".to_string(), "我发了一条微博".to_string());
    notify(&post);
    notify(&weibo);
    // notify("hello");//Trait `Summary` is not implemented for `str` [E0277]
}

/// # 泛型参数的语法糖
/// # Example
///```
/// pub fn notify<T: Summary>(item: &T) {
///     println!("Breaking news! {}", item.summarize());
/// }
///```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// # 特征约束
fn trait_bound() {
    let post = Post::new("Head first Rust".to_string(), "Zhong".to_string(), "一本介绍Rust的书籍".to_string());
    let post2 = Post::new("Head first Golang".to_string(), "Zhong".to_string(), "一本介绍Go的书籍".to_string());
    let weibo = Weibo::new("重".to_string(), "我发了一条微博".to_string());

    notify2(&post, &post2);
    // notify2(&post, &weibo); //type mismatch [E0308] expected `&Post`, but found `&Weibo`
    notify3(&post2, &weibo);
    notify3(&post, &post2);

    // 多重约束 Summary + Display
    //notify4(&weibo); //Trait `Display` is not implemented for `Weibo` [E0277]
    notify4(&post);

    // 多重约束 类型+泛型
    // notify("Hello"); // Trait `Summary` is not implemented for `str` [E0277]
    notify5(&post);
}

/// 同时限制类型和Trait
pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("{} {}", item1.summarize(), item2.summarize());
}

/// 没有类型限制，只需要实现 Summary Trait
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("{} {}", item1.summarize(), item2.summarize());
}

/// 多重约束
pub fn notify4(item: &(impl Summary + Display)) {
    println!("{}", item.summarize()); //Summary 特征方法
    println!("{}", item); // Display 格式化输出 fmt
}


/// 多重约束 : 类型T + 泛型 Summary 和 Display
pub fn notify5<T: Summary + Display>(item: &T) {
    println!("{}", item.summarize()); //Summary 特征方法
    println!("{}", item); // Display 格式化输出 fmt
}

/// # Where 约束
fn where_bound() {
    let t = "Hello,Rust";
    let u = vec![1, 2, 3];
    let result = some_function(&t, &u);
    println!("The length of U's debug output is: {}", result);

    let result2 = some_where_function(&t, &u);
    println!("The length of U's debug output is: {}", result2);
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("Displaying T: {}", t);

    let debug_output = format!("{:?}", u);
    debug_output.len() as i32
}

fn some_where_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    println!("Displaying T: {}", t);

    let debug_output = format!("{:?}", u);
    debug_output.len() as i32
}

/// # 特征约束，可以让我们在指定类型 + 指定特征的条件下去实现方法
fn condition_bound() {
    let p = Pair::new(1.0, 2.0);
    p.cmp_display();

    let c = Pair::new('z', 'a');
    c.cmp_display();
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/// # 函数返回中的 impl Trait
fn return_impl_trait() {
    println!("{}", returns_summarize().summarize());
    // println!("{}", returns_summarizable(true).summarize()); //error[E0308]: `if` and `else` have incompatible types
}

/// 虽然是返回的 Trait 。但是对类型是有要求的，必须是同一个类型
fn returns_summarize() -> impl Summary {
    Weibo {
        username: "重".to_string(),
        content: "Trait 作为返回值".to_string(),
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

/// # 为外部类型实现外部特征
/// 绕过孤儿原则
/// 
/// # 为 Vec<T> 实现外部特征 
/// ```
/// // Vec` is not defined in the current crate
/// impl<T> Display for Vec<T> {
/// } 
/// ``` 
/// 可以使用newtype 绕过这个限制 
/// 
fn new_type(){
    let w = Wrapper(vec!["hello".to_string(),"tuple struct".to_string()]);
    println!("{}",w)
}

// 元组结构体
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"[{}]",self.0.join(","))
    }
}