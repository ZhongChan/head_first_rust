mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use collisions::collisions_system;
use end_turn::end_turn_system;
use entity_render::entity_render_system;
use map_render::map_render_system;
use player_input::player_input_system;
use random_move::random_move_system;

use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .flush()
        .add_system(collisions_system())
        .add_system(random_move_system())
        .build()
}

pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}

pub fn build_player_schedule() -> Schedule {
    Schedule::builder()
        .add_system(collisions_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_schedule() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .flush()
        .add_system(collisions_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}
