use crate::prelude::*;

/// 房间数量
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub roms: Vec<Rect>, //Rect 处理矩形相关运算
    pub player_start: Point, //玩家初始位置
}

impl MapBuilder {
    /// # 房屋开凿算法
    /// * `iter_mut` 获取一个可变迭代器 
    /// * `for_each` 把每一个图块类型设置成指定类型
    /// * `t` 前面的 星号（`*`） 是`解引用`
    ///     * 迭代器传递的变量 `t`是一个可变引用，也就是 `&mut TileType`
    ///     * `解引用`表示开发者想修改被引用的变量，而不是修改引用本身
    pub fn fill(&mut self, tile_type: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile_type);
    }
}

