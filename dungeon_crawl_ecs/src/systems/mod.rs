mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use collisions::collisions_system;
use random_move::random_move_system;

use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::player_render_system())
        .flush()
        .add_system(collisions_system())
        .add_system(random_move_system())
        .build()
}
