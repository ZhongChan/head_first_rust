mod map;
mod player;

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
