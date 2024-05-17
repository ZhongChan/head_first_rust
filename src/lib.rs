#[macro_export]
macro_rules! print_size_of_val {
    ($var:ident) => {
        println!("size of {}: {} bytes",stringify!($var),std::mem::size_of_val(&$var));
    };
}

#[macro_export]
macro_rules! print_size_of_char {
    ($var:ident) => {
        println!("字符'{}'占用了 {} 个字节",stringify!($var),std::mem::size_of_val(&$var));
    };
}