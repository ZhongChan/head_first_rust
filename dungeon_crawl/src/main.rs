mod map;
mod player;

/// # 使用其他模块
/// 模块之间是通过树形结构来组织的，使用 `use` 关键字导入时：
/// * `super::` 树形结构中使用位于自己相邻上级的模块。
/// * `crate::` 访问位于树根的模块，也就是 `main.rs`。
mod prelude {
    pub use bracket_lib::prelude::*;

    const SCREEN_WIDTH: i32 = 80;
    const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map::*;
}

use prelude::*;

fn main() {
    println!("Hello, world!");
}
