use crate::map::TileType::Floor;
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

    pub fn build_random_roms(&mut self, rng: &mut RandomNumberGenerator) {

        // 生成 NUM_ROOMS 个非相交房间
        while self.roms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            //是否有房间重叠在一起
            let mut overloop = false;
            for r in self.roms.iter() {
                if r.intersect(&room) {
                    overloop = true;
                }
            }

            // 如果房间不重叠，检查其中每一个点是否都在地图范围内。如果是，就把对应位置改成壁板。
            if !overloop {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = Floor
                    }
                });
                self.roms.push(room)
            }
        }
    }
}

