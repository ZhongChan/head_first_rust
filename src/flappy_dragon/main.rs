use bracket_lib::prelude::*;
use crate::GameMode::{End, Menu, Playing};

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
        match self.mode {
            Menu => { self.main_menu(ctx) }
            Playing => { self.play(ctx) }
            End => { self.dead(ctx) }
        }
    }
}

impl State {
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO
        self.mode = End
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO
        self.mode = Playing
    }

    fn end(&mut self, ctx: &mut BTerm) {
        // TODO
        self.mode = Menu
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