use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
/// # Heads Up Display
/// * `(1)` Define a query that reads `Health` components,filterd to show only the `Player`
/// * `(2)` Return the `n`th elements like `arr[0]`
/// * `(3)` Draw to the HUD console layer
/// * `(4)` Greet the player and give them and idea of how to play
pub fn hud(ecs: &mut SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>()); //(1)
    let player_health = health_query
        .iter(ecs)
        .nth(0) //(2)
        .unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2); //(3)
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move."); //(4)
    draw_batch.bar_horizontal(
        //(5)
        Point::zero(), //(6)
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.submit(10000).expect("Batch error");
}
