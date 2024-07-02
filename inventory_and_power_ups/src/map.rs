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
    Exit,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_titles: Vec<bool>,
}

impl Map {
    ///  构造一个全地板地图
    pub fn new() -> Self {
        Self {
            tiles: vec![Floor; NUM_TILES],
            revealed_titles: vec![false; NUM_TILES],
        }
    }
}

impl Map {
    /// 是否在地图内
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// 能否进入图块
    /// 在地图内，且图块是地板
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && (self.tiles[map_idx(point.x, point.y)] == Floor
                || self.tiles[map_idx(point.x, point.y)] == TileType::Exit)
    }

    /// 尝试获取图块
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// # 为地图创建索引
/// 使用行优先的编码方式，通过坐标获取地图索引
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == Floor
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1), // (1)
            self.index_to_point2d(idx2), // (2)
        )
    }
}
