use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TrunState) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    let current_state = turn_state.clone();

    let mut new_state = match turn_state {
        TrunState::AwaitingInput => return,
        TrunState::PlayerTurn => TrunState::MonsterTurn,
        TrunState::MonsterTurn => TrunState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TrunState::GameOver;
        }

        if pos == amulet_pos {
            new_state = TrunState::Victory;
        }
    });

    *turn_state = new_state;
}
