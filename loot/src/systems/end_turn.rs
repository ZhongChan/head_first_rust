use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn end_turn(
    ecs: &SubWorld, // (1)
    #[resource] turn_state: &mut TrunState,
    #[resource] map: &Map,
) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap_or(&amulet_default);

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

        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TrunState::NextLevel;
        }
    });

    *turn_state = new_state;
}
