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
    // 添加调试输出以确认当前工作目录和资源路径
    println!("Current working directory: {:?}", std::env::current_dir()?);
    println!("Expected resource path: resources/dungeonfont.png");

    // 创建了一个终端窗口
    // 包含两个控制图层：一个用来绘制地图，一个用来绘制角色。
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawl")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) //控制台尺寸
        .with_tile_dimensions(32, 32) //图块尺寸
        .with_resource_path("resources/") //资源
        .with_font("dungeonfont.png", 32, 32) //要加载的字体文件，尺寸和图块尺寸保持一致 (高级用法可以不一致)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") //增加一个新的控制台图层
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") //新增的控制台图层，没有背景色
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
