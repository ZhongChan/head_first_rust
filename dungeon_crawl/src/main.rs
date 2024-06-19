mod map;
mod map_builder;
mod player;

/// # 使用其他模块
/// 模块之间是通过树形结构来组织的，使用 `use` 关键字导入时：
/// * `super::` 树形结构中使用位于自己相邻上级的模块。
/// * `crate::` 访问位于树根的模块，也就是 `main.rs`。
mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

fn main() -> BResult<()> {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawl")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}

struct State {
    map: Map,
    player: Player,
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mp = MapBuilder::new(&mut rng);
        Self {
            map: mp.map,
            player: Player::new(mp.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
