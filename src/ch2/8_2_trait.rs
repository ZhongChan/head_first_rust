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
    let post = Post::new("Head first Rust".to_string(), "Zhong".to_string(), "一本介绍Rust的书籍".to_string());
    println!("{}", post.summarize());
    let weibo = Weibo::new("重".to_string(), "我发了一条微博".to_string());
    println!("{}", weibo.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String;
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

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章：{}，作者：{}", self.title, self.author)
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
