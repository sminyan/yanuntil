// src/lib.rs
///现有数字加1
/// #例子
/// assert_eq!(2,my_crate::add_one(1));
///
pub fn add_one(x:i32)->i32{
    x+1
}

#[macro_export]
macro_rules! my_print {
    ($(arg:tt)*) => {
        let output = format!($($arg)*);
        std::io::stdout().write_all(output.as_bytes()).expect("Failed to write to stdout");
    };
}