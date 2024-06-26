use crate::prelude::*;

#[system]
#[read_component(Health)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TrunState) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state = turn_state.clone();

    let mut new_state = match turn_state {
        TrunState::AwaitingInput => return,
        TrunState::PlayerTurn => TrunState::MonsterTurn,
        TrunState::MonsterTurn => TrunState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|hp| {
        if hp.current < 1 {
            new_state = TrunState::GameOver;
        }
    });

    *turn_state = new_state;
}
