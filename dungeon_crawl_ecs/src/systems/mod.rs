mod collisions;
mod entity_render;
mod map_render;
mod player_input;

use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::player_render_system())
        .build()
}
