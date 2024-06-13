#[macro_export]
macro_rules! print_size_of_val {
    ($var:ident) => {
                println!("'{}'占用了 {} 个字节",stringify!($var),std::mem::size_of_val(&$var));
    };
}

#[macro_export]
macro_rules! print_size_of_char {
    ($var:ident) => {
        println!("'{}'占用了 {} 个字节",stringify!($var),std::mem::size_of_val(&$var));
    };
}

#[macro_export]
macro_rules! generate_main {
    ( $( ($name:expr, $func:expr) ),* ) => {
        fn main() {
            let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
                $(
                    ($name, Box::new(|| $func()))
                ),*
            ];

            for (name, function) in functions.into_iter() {
                println!();
                println!(">>>>>>>>>>开始执行：{}", name);
                function();
                println!("{}: 执行结束<<<<<<<<<<", name);
            }
        }
    };
}
