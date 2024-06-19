use crate::prelude::*;

/// 房间数量
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub roms: Vec<Rect>,
    pub player_start: Point,
}