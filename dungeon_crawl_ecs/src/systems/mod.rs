mod player_input;

use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}