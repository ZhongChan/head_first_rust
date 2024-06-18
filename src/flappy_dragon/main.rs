use bracket_lib::prelude::*;
use crate::GameMode::Menu;

fn main() -> BResult<()> {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(ctx, State::new())
}

struct State {
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        Self { mode: Menu }
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx.cls();
        // ctx.print(1, 1, "Hello, Bracket Terminal!");
        match self.mode {
            Menu => { self.main_menu(ctx) }
            GameMode::Playing => { self.play(ctx) }
            GameMode::End => { self.dead(ctx) }
        }
    }
}


/// # 游戏模态
/// * GameMode 是一个枚举
/// * 要么显示菜单、要么在进行游戏、要么游戏结束
enum GameMode {
    Menu,
    Playing,
    End,
}