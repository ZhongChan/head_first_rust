use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TrunState) {
    let new_state = match turn_state {
        TrunState::AwaitingInput => return,
        TrunState::PlayerTurn => TrunState::MonsterTurn,
        TrunState::MonsterTurn => TrunState::PlayerTurn,
    };

    *turn_state = new_state;
}
