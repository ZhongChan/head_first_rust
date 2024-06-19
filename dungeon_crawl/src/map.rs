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

/// # 为地图创建索引
/// 使用行优先的编码方式，通过坐标获取地图索引
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

/// 通过索引反算坐标
pub fn map_x(idx: usize) -> i32 {
    (idx as i32) % SCREEN_WIDTH
}

pub fn map_y(idx: usize) -> i32 {
    (idx as i32) / SCREEN_WIDTH
}