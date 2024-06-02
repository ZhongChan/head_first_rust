use std::fmt::{Display, Formatter};

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("孤儿规则", Box::new(|| orphan_rule())),
        ("使用特征作为函数参数", Box::new(|| trait_as_params())),
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
        format!("{}发表了微博{}", self.username, self.content)
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

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}