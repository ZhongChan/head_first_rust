use crate::prelude::*;

#[system]
#[read_component(WantsToAttact)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attacters = <(Entity, &WantsToAttact)>::query();
    let victims: Vec<(Entity, Entity)> = attacters
        .iter(ecs)
        .map(|(entity, attact)| (*entity, attact.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attact: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            println!("Health after attact: {}", health.current);
        };
        commands.remove(*message);
    });
}
