use crate::GameMode::{End, Menu, Playing};
use bracket_lib::prelude::*;

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
            Menu => self.main_menu(ctx),
            Playing => self.play(ctx),
            End => self.dead(ctx),
        }
    }
}

impl State {
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.play(ctx),
                VirtualKeyCode::Q => ctx.quitting = true, //退出
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = End
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),      //重启
                VirtualKeyCode::Q => ctx.quitting = true, //退出
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
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

/// # 玩家
/// * x 坐标，世界坐标系
/// * y 子啊世界坐标系下竖直方向的位置
/// * velocity 竖直方向的速度
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }
}
