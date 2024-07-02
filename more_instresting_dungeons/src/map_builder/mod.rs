use crate::map::TileType::Floor;
use crate::prelude::*;
use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use rooms::RoomsArchitect;
use std::vec;

mod automata;
mod drunkard;
mod empty;
mod rooms;

/// 房间数量
const NUM_ROOMS: usize = 20;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,    //Rect 处理矩形相关运算
    pub player_start: Point, //玩家初始位置
    pub amulet_start: Point,
    pub monster_spawns: Vec<Point>,
}

impl MapBuilder {
    /// 建造房间并放置玩家
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };
        architect.new(rng)
    }
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

    pub fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    pub fn build_random_roms(&mut self, rng: &mut RandomNumberGenerator) {
        // 生成 NUM_ROOMS 个非相交房间
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            //是否有房间重叠在一起
            let mut overloop = false;
            for r in self.rooms.iter() {
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
                self.rooms.push(room)
            }
        }
    }

    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = Floor;
            }
        }
    }

    pub fn apply_horizon_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = Floor;
            }
        }
    }

    pub fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizon_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizon_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    fn spawner_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns: Vec<Point> = Vec::new();

        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
}
