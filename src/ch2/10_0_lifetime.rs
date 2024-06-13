use std::borrow::Cow;
use std::ptr;

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("悬垂引用和生命周期", Box::new(|| dangle_ref_lifetime())),
        ("函数中的生命周期", Box::new(|| function_lifetime())),
        ("生命周期标注语法", Box::new(|| lifetime_tag())),
        ("结构体中的生命周期", Box::new(|| struct_lifetime())),
        ("生命周期消除", Box::new(|| lifetime_elision())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}


/// 在 Rust 中，“引用”和“借用”是密切相关的概念，但它们并不完全是同一个意思。以下是对这两个术语的解释：
/// ## 引用（Reference）
/// 引用是指向某个值的指针。Rust 中的引用有两种类型：
/// * 不可变引用 (&T)：允许你读取数据但不能修改数据。
/// * 可变引用 (&mut T)：允许你读取和修改数据。
///
/// 引用的主要目的是为了避免数据的复制，通过引用可以在多个地方访问同一个数据。
///
/// ## 借用（Borrowing）
/// 借用是指将某个值的引用传递给另一个作用域。借用可以是不可变借用或可变借用：
/// * 不可变借用：当你通过不可变引用访问数据时，称之为不可变借用。一个值可以有多个不可变借用。
/// * 可变借用：当你通过可变引用访问数据时，称之为可变借用。一个值在任意时刻只能有一个可变借用，且不能与不可变借用同时存在。
///
/// ## 借用规则
/// 借用规则由 Rust 编译器强制执行，以确保数据的安全性和防止数据竞争（data race）。具体规则包括：
/// * 在任意时刻，只能有一个可变引用，或者任意数量的不可变引用。
/// * 引用必须始终有效。
///
fn basic() {
    let x = 5;

    // 不可变引用
    let r1 = &x;
    let r2 = &x;
    println!("r1: {}, r2 :{}", r1, r2); // 允许多个不可变引用

    // 可变引用
    let mut y = 6;
    let r3 = &mut y;
    println!("r3:{}", r3);
}

/// # 悬垂引用
/// 悬垂引用（dangling reference）指的是一个引用指向了已经被释放或移除的内存位置
/// # 参考
/// - [`dangle_ref`]
/// todo 优化文档引用
fn dangle_ref_lifetime() {
    let r;
    {
        let x = 5;
        r = &x; //error[E0597]: `x` does not live long enough
        println!("{}", r)
    }
    // println!("{}", r)
}

fn function_lifetime() {
    let s1 = String::from("Lifetime");
    let s2 = "Rust";
    // let result = longest_dangle(s1.as_str(), s2); //Process finished with exit code 139 (interrupted by signal 11:SIGSEGV) 访问被释放的内存
    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);
}

/// # 生命周期
/// 生命周期参数 'a：
/// * 我们在 longest 函数中添加了生命周期参数 'a，
/// * 这表示 s1 和 s2 的引用必须在同一个生命周期内，
/// * 并且返回的引用也将具有相同的生命周期。
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/// # 使用 unsafe 破坏内存
/// ## 解释
/// 1. Box::new：我们使用 Box 来分配在堆上的字符串，这样我们可以手动释放它。
/// 2. unsafe 块：使用 unsafe 块绕过 Rust 的借用检查器。
/// 3. 指针转换：将 s1 转换为一个原始指针 *const str。
/// 4. 手动释放：通过 ptr::drop_in_place 手动释放 s1 所指向的 Box<String>，这会使 s1 成为悬垂引用。
/// 5. 悬垂引用：尝试通过原始指针 s1_ptr 访问已经被释放的内存。
#[allow(dead_code)]
fn longest_dangle<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    unsafe {
        // 将 s1 转换为一个原始指针
        let s1_ptr = s1 as *const str;

        // 手动释放 s1 所指向的 Box<String>
        let s1_boxed = s1_ptr as *const Box<String>;
        ptr::drop_in_place(s1_boxed as *mut Box<String>);

        // 尝试通过原始指针访问已经被释放的内存
        if (*s1_ptr).len() > s2.len() {
            &*s1_ptr
        } else {
            s2
        }
    }
}

/// # 生命周期标注语法
/// `生命周期标注并不会改变任何引用的实际作用域`
///
pub fn lifetime_tag() {
    lifetime_the_little();
    lifetime_the_little_wrong();
    lifetime_one_params();
    lifetime_dangle();
}

/// ## 返回参会生命周期等于请求参数中较小那个
fn lifetime_the_little() {
    let long = "long string is long".to_string();
    {
        let short = "short".to_string();
        let result = longest(long.as_str(), short.as_str());
        println!("The longest string is: {}", result);
    }
}

/// ## 返回参会生命周期等于请求参数中较小那个
/// ## 错误示例
/// ```rust
/// fn lifetime_the_little_wrong() {
///     let long = "long string is long".to_string();
///     let result;
///     {
///         let short = "short".to_string();
///         result = longest(long.as_str(), short.as_str());
///     } //     `short` dropped here while still borrowed
///     println!("The longest string is: {}", result);
/// }
/// ```
///
/// ## 编译时报错：
/// ```text
///error[E0597]: `short` does not live long enough
///    --> src/ch2/10_0_lifetime.rs:156:41
///     |
/// 155 |         let short = "short".to_string();
///     |             ----- binding `short` declared here
/// 156 |         result = longest(long.as_str(), short.as_str());
///     |                                         ^^^^^ borrowed value does not live long enough
/// 157 |     } //     `short` dropped here while still borrowed
///     |     - `short` dropped here while still borrowed
/// 158 |     println!("The longest string is: {}", result);
///     |                                           ------ borrow later used here
/// ```
///
/// ## 错误原因
/// 1. *生命周期参数* `'a：longest` 函数的生命周期参数 `'a` 表示返回值的生命周期必须与两个输入引用的生命周期中较短的那个相同。
/// 2. *编译器无法判断*：在 `longest` 函数内部，编译器无法判断 `result` 引用的是 `long` 还是 `short`。因此，为了安全起见，编译器假设 `result` 的生命周期与 `short` 一样长。
/// 3. *块作用域*：`short` 在块结束时被释放，而 `result` 可能引用了 `short`，这就导致了潜在的悬垂引用。
///
///
fn lifetime_the_little_wrong() {
    let long = "long string is long".to_string();
    let result;
    {
        let short = "short".to_string();
        result = longest(long.as_str(), short.as_str());
        println!("The longest string is: {}", result);
    }
}

/// # 返回参数只和单一参数相关
/// ## 错误示例
/// ```rust
/// fn lifetime_one_params() {
///     let long = "long string is long".to_string();
///     let result;
///     {
///         let short = "short".to_string();
///         result = longest_one(short.as_str(), long.as_str());
///     }
///     println!("The longest string is: {}", result);
/// }
///
///
/// fn longest_one<'a>(s1: &'a str, _s2: &str) -> &'a str {
///     s1
/// }
/// ```
/// ## 编译错误
/// ```text
/// error[E0597]: `short` does not live long enough
///    --> src/ch2/10_0_lifetime.rs:185:30
///     |
/// 184 |         let short = "short".to_string();
///     |             ----- binding `short` declared here
/// 185 |         result = longest_one(short.as_str(), long.as_str());
///     |                              ^^^^^ borrowed value does not live long enough
/// 186 |     }
///     |     - `short` dropped here while still borrowed
/// 187 |     println!("The longest string is: {}", result);
///     |                                           ------ borrow later used here
/// ```
///
/// ## 结论
/// 编译器推断出了谁的生命周期更长，但是这种写法相当危险。
///
fn lifetime_one_params() {
    let long = "long string is long".to_string();
    let result;
    {
        let short = "short".to_string();
        // result = longest_one(short.as_str(), long.as_str()); //error[E0597]: `short` does not live long enough
        result = longest_one(long.as_str(), short.as_str());
    }
    println!("The longest string is: {}", result);
}


fn longest_one<'a>(s1: &'a str, _s2: &str) -> &'a str {
    s1
}


/// # 悬垂引用
/// ## 错误示例
/// ```rust
/// fn lifetime_dangle() {
///     let long = "long string is long".to_string();
///     let result;
///     {
///         let short = "short".to_string();
///         result = longest_from_string(long.as_str(), short.as_str());
///         println!("The longest string is: {}", result);
///     }
/// }
///
/// fn longest_from_string<'a>(_x: &str, _y: &str) -> &'a str {
///     let result = "really long string".to_string();
///     result.as_str()
/// }
/// ```
/// ## 编译错误
/// ```text
/// error[E0515]: cannot return value referencing local variable `result`
///    --> src/ch2/10_0_lifetime.rs:230:5
///     |
/// 230 |     result.as_str()
///     |     ------^^^^^^^^^
///     |     |
///     |     returns a value referencing data owned by the current function
///     |     `result` is borrowed here
/// ```
fn lifetime_dangle() {
    let long = "long string is long".to_string();
    let result;
    {
        let short = "short".to_string();
        result = longest_from_string(long.as_str(), short.as_str());
        println!("The longest string is: {}", result);
    }
}

fn longest_from_string<'a>(_x: &str, _y: &str) -> String {
    "really long string".to_string()
}

/// # 结构体中的生命周期
fn struct_lifetime() {
    struct_lifetime_base();
    struct_lifetime_wrong();
}

/// # 结构体生命周期：基础示例
/// ```rust
/// struct ImportantExcerpt {
///     part: &str, //Missing lifetime specifier [E0106]
/// }
/// ```
///
/// * `ImportantExcerpt` 结构体中有一个引用类型的字段 `part`，因此需要为它标注上生命周期。
/// * 结构体的生命周期标注语法跟泛型参数语法很像，需要对生命周期参数进行声明 `<'a>`。
/// * 该生命周期标注说明，结构体 `ImportantExcerpt` 所引用的字符串 `str` 必须比该结构体活得更久。
///
fn struct_lifetime_base() {
    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split('.').next().expect("Could not found a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

/// # 结构体生命周期：错误示例
/// ## 代码
/// ```rust
/// fn struct_lifetime_wrong() {
///     let i;
///     {
///         let novel = "Call me Ishmael. Some years ago...".to_string();
///         let first_sentence = novel.split('.').next().expect("Could not found a '.'");
///         i = ImportantExcerpt {
///             part: first_sentence,
///         };
///     }
///     println!("{}", i.part);
/// }
///
/// struct ImportantExcerpt<'a> {
///     part: &'a str,
/// }
/// ```
/// ## 编译错误
/// ```text
/// error[E0597]: `novel` does not live long enough
///    --> src/ch2/10_0_lifetime.rs:306:30
///     |
/// 305 |         let novel = "Call me Ishmael. Some years ago...".to_string();
///     |             ----- binding `novel` declared here
/// 306 |         let first_sentence = novel.split('.').next().expect("Could not found a '.'");
///     |                              ^^^^^ borrowed value does not live long enough
/// ...
/// 310 |     }
///     |     - `novel` dropped here while still borrowed
/// 311 |     println!("{}", i.part);
///     |                    ------ borrow later used here
/// ```
///
fn struct_lifetime_wrong() {
    let i; //结构体生命周期长
    let novel = "Call me Ishmael. Some years ago...".to_string();
    {
        // let novel = "Call me Ishmael. Some years ago...".to_string(); //被结构体引用
        let first_sentence = novel.split('.').next().expect("Could not found a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{}", i.part);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// # 生命周期消除
/// 在 Rust 中，生命周期消除（lifetime elision）是指编译器在某些情况下自动推断生命周期，而不需要程序员显式地注解生命周期。
/// 这种机制使得代码更简洁，同时保持了 Rust 的内存安全性。Rust 语言的设计者为了简化常见代码模式，引入了一些生命周期消除规则。
///
/// ## 生命周期消除规则
/// Rust 编译器在以下情况下会应用生命周期消除规则：
/// 1. 输入生命周期：
///     * 如果一个函数只有一个输入生命周期参数，那么这个生命周期会被赋给所有的输出生命周期参数。
///     * 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，那么 self 的生命周期会被赋给所有的输出生命周期参数。
/// 2. 多输入生命周期：
///     * 如果有多个输入生命周期参数，但没有 &self 或 &mut self，编译器无法自动推断输出生命周期，因此需要显式地指定生命周期。
///
/// ## 具体规则
/// 1. 单输入引用：
/// ```rust
/// fn first_word(s: &str) -> &str {
///     // 函数体
/// }
/// //编译器推断为
/// fn first_word<'a>(s: &'a str) -> &'a str {
///     // 函数体
/// }
/// ```
/// 2. 多输入引用，有 `&self` 或 `&mut self`：
/// ```rust
/// impl MyStruct {
///     fn get_value(&self, key: &str) -> &str {
///         // 函数体
///     }
/// }
///
/// //编译器推断为
///  impl MyStruct {
///     fn get_value<'a>(&'a self, key: &'a str) -> &'a str {
///         // 函数体
///     }
/// }
///
/// ```
/// 3. 多输入引用，无 `&self` 或 `&mut self`：
/// ```rust
///  fn longest(x: &str, y: &str) -> &str // error[E0106]: missing lifetime specifier
///
///  // 需要显示声明周期
///  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
/// ```
fn lifetime_elision() {
    // 单输入
    println!("单输入引用：{}", lifetime_elision_single("Rust"));

    // 多输入无&self
    let long = "The long string".to_string();
    let short = "short".to_string();
    let the_long = lifetime_elision_multi(long.as_str(), short.as_str());
    println!("多输入引用无&self：{}", the_long);

    // 多输入无&self，多周期
    let string1 = String::from("short");
    let string2 = String::from("a bit longer");
    let string3 = String::from("the longest string of all");

    let result = select_shortest(&string1, &string2, &string3);
    println!("The shortest string is: {}", result);
}

/// # 生命周期消除：单输入引用
fn lifetime_elision_single(s: &str) -> &str {
    let s_bytes = s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

///  # 生命周期消除：多输入引用无&self
/// ## 生命周期不统一
/// ```rust
/// fn lifetime_elision_multi<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
///     if s1.len() > s2.len() {
///         s1
///     } else {
///         s2
///     }
/// }
/// ```
///
/// ## 解决方案：统一生命周期
/// ```rust
/// fn lifetime_elision_multi<'a>(s1: &'a str, s2: &'a str) -> &'a str {
///     if s1.len() > s2.len() {
///         s1
///     } else {
///         s2
///     }
/// }
/// ```
///

fn lifetime_elision_multi<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/// # 命周期消除：多输入引用无&self，对齐到最短
/// 在这个例子中：
/// 1. `select_shortest` 函数接受三个字符串引用，分别具有生命周期 `'a, 'b, 和 'c`。
/// 2. 通过对齐最短的生命周期，我们确保返回的引用是安全的。
/// 3. 使用 `Cow` 类型，如果返回的引用不是最短生命周期的引用，我们会将其克隆为 `Cow::Owned`，以确保返回值的生命周期 'a 足够长。
fn select_shortest<'a, 'b, 'c>(s1: &'a str, s2: &'b str, s3: &'c str) -> Cow<'a, str>
    where
        'b: 'a,
        'c: 'a,
{
    let shortest = if s1.len() < s2.len() {
        if s1.len() < s3.len() {
            s1
        } else {
            s3
        }
    } else {
        if s2.len() < s3.len() {
            s2
        } else {
            s3
        }
    };

    // If the shortest string does not have the shortest lifetime, we need to clone it.
    if shortest == s1 {
        Cow::Borrowed(s1)
    } else {
        Cow::Owned(shortest.to_string())
    }
}