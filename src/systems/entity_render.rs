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
            let draw_params = DrawTextureParams {
                dest_size: DEST_SIZE,
                source: Some(render.texture.get_texture_source()),
                ..Default::default()
            };

            draw_texture_ex(
                &sprite_sheet.entities,
                viewport.get_screen_x(pos.x),
                viewport.get_screen_y(pos.y),
                WHITE,
                draw_params,
            );
        }
    }
}
