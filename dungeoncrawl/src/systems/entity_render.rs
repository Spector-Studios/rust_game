use crate::prelude::*;

pub fn entity_render_system(
    viewport: Res<Viewport>,
    sprite_sheet: Res<SpriteSheet>,
    entity_query: Query<(&TilePoint, &Render)>,
    fov: Query<&FieldOfView, With<Player>>,
) {
    let player_fov = fov.single().unwrap();

    for (pos, render) in entity_query.iter() {
        if viewport.view_area.contains(*pos) && player_fov.visible_tiles.contains(&(*pos).into()) {
            draw_texture_ex(
                sprite_sheet.sprites.get(&render.texture).unwrap(),
                viewport.get_screen_x(pos.x),
                viewport.get_screen_y(pos.y),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );
        }
    }
}
