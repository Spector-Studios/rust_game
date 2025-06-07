use crate::prelude::*;

pub fn entity_render_system(
    viewport: Res<Viewport>,
    sprite_sheet: Res<SpriteSheet>,
    entity_query: Query<(&TilePoint, &Render)>,
) {
    for (pos, render) in entity_query.iter() {
        if viewport.view_area.contains(*pos) {
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
