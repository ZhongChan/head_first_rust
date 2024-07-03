use crate::prelude::*;

#[system]
#[read_component(WantsToAttact)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Wepon)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attacters = <(Entity, &WantsToAttact)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attacters
        .iter(ecs)
        .map(|(entity, attact)| (*entity, attact.attacter, attact.victim))
        .collect();

    victims.iter().for_each(|(message, attacter, victim)| {
        // attacter damage
        let base_dmg = if let Ok(v) = ecs.entry_ref(*attacter) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_dmg: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carreid, _)| carreid.0 == *attacter)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let final_dmg = base_dmg + weapon_dmg;

        //player's health zero
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attact: {}", health.current);
            health.current -= final_dmg;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("Health after attact: {}", health.current);
        };
        commands.remove(*message);
    });
}
