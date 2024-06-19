use crate::GameMode::{End, Menu, Playing};
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() -> BResult<()> {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(ctx, State::new())
}

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

impl State {
    pub fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: Menu,
            score: 0,
        }
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
        self.mode = Playing;
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");

        //得分
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        //超出屏幕或者碰撞障碍物
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earn points: {}", self.score));
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
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = Playing;
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

impl Player {
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        // 下落速度小于2添加重力
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32; //向下取整
        self.x += 1;
        if self.y < 0 {
            self.y = 0
        }
    }

    // 煽动翅膀
    fn flap(&mut self) {
        self.velocity = -2.0
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut generator = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: generator.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        //绘制障碍物上半部分
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        //绘制障碍物下半部分
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&mut self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let dose_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_bellow_gap = player.y > self.gap_y + half_size;
        dose_x_match && (player_above_gap || player_bellow_gap)
    }
}