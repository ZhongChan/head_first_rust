use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(
    ecs: &SubWorld,              // (1)
    #[resource] map: &Map,       // (2)
    #[resource] camera: &Camera, // (3)
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            // in map and player can see the field
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Wall => to_cp437('#'),
                    TileType::Floor => to_cp437('.'),
                };

                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error")
}
