pub mod ch2;

#[macro_export]
macro_rules! print_size_of_val {
    ($var:expr) => {
        println!("'{}' 占用了: {} 个字节", stringify!($var), std::mem::size_of_val(&$var));
    };
}

#[macro_export]
macro_rules! print_size_of_char {
    ($var:expr) => {
        println!("'{}' 占用了 {} 个字节", stringify!($var), std::mem::size_of_val(&$var));
    };
}
