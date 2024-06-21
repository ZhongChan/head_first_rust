mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

/// # 使用其他模块
/// 模块之间是通过树形结构来组织的，使用 `use` 关键字导入时：
/// * `super::` 树形结构中使用位于自己相邻上级的模块。
/// * `crate::` 访问位于树根的模块，也就是 `main.rs`。
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_system: Schedule,
    player_system: Schedule,
    monster_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawner_player(&mut ecs, map_builder.player_start);

        // spawner one monster per room
        map_builder
            .rooms
            .iter()
            .skip(1) //跳过第一个房间
            .map(|r| r.center()) //transformer each entry from a room to result of `center` (a `Point`) use `map()`
            .for_each(|pos| spaner_monster(&mut ecs, &mut rng, pos));

        // 地图和摄像机都是资源
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TrunState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_system: build_input_schedule(),
            player_system: build_player_schedule(),
            monster_systems: build_monster_schedule(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // map
        ctx.set_active_console(0);
        ctx.cls();

        // player and monster
        ctx.set_active_console(1);
        ctx.cls();

        // health
        ctx.set_active_console(2);
        ctx.cls();

        // 资源：键盘输入
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // Execute Systems
        let current_state = self.resources.get::<TrunState>().unwrap().clone();
        match current_state {
            TrunState::AwaitingInput => self
                .input_system
                .execute(&mut self.ecs, &mut self.resources),

            TrunState::PlayerTurn => self
                .player_system
                .execute(&mut self.ecs, &mut self.resources),

            TrunState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
        }

        // Render Draw Buffer
        render_draw_buffer(ctx).expect("Render error")
    }
}

fn main() -> BResult<()> {
    // 添加调试输出以确认当前工作目录和资源路径
    println!("Current working directory: {:?}", std::env::current_dir()?);
    println!("Expected resource path: resources/dungeonfont.png");

    let context = BTermBuilder::new()
        .with_title("Dungeon Crawl")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) //控制台尺寸
        .with_tile_dimensions(32, 32) //图块尺寸
        .with_resource_path("resources/") //资源
        .with_font("dungeonfont.png", 32, 32) //要加载的字体文件，尺寸和图块尺寸保持一致 (高级用法可以不一致)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") //增加一个新的控制台图层 map
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") //新增的控制台图层，没有背景色 player and monster
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png") //health
        .build()?;
    main_loop(context, State::new())
}