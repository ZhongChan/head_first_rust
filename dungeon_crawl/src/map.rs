use crate::map::TileType::Floor;
use crate::prelude::*;

/// # 地图中图块（tile）数量常量
/// * `usize` ： 和当前 `cpu` 架构保持一致 64或32
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/// # 图块枚举
/// * 墙壁
/// * 地板
/// * 。。。
///
/// `derive` 派生说明：
/// * `Clone`: 使用 my_title.clone() 会创建深拷贝。
/// * `Copy`: `TileType`赋值时，不再转移所有权，而是拷贝。
/// * `PartialEq`: `暗中`添加一些代码，可以使用 `==` 比较两个 `TileType` 类型的变量。
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    ///  构造一个全地板地图
    pub fn new() -> Self {
        Self {
            tiles: vec![Floor; NUM_TILES],
        }
    }
}

impl Map {
    /// # 渲染地图
    /// * `地板`：黄色 `.`
    /// * `墙壁`：绿色 `#`
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0); // 地图绘制在第一个图层
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                // 是否在地图内
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                        Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                    }
                }
            }
        }
    }

    /// 是否在地图内
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// 能否进入图块
    /// 在地图内，且图块是地板
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == Floor
    }

    /// 尝试获取图块
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

/// # 为地图创建索引
/// 使用行优先的编码方式，通过坐标获取地图索引
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
