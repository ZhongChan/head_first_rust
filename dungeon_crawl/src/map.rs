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