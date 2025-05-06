use crate::camera::Camera;
use crate::prelude::*;

pub fn entity_render_system(
    camera: Res<Camera>,
    sprite_sheet: Res<SpritesStore>,
    entity_query: Query<(&TilePoint, &Render)>,
) {
    for (pos, render) in entity_query.iter() {
        if camera.view_area.contains(*pos) {
            draw_texture_ex(
                &sprite_sheet.sprites,
                camera.get_screen_x(pos.x),
                camera.get_screen_y(pos.y),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    source: Some(render.texture_source),
                    ..Default::default()
                },
            );
            /* draw_rectangle(
                camera.get_screen_x(pos.x),
                camera.get_screen_y(pos.y),
                TILE_SIZE,
                TILE_SIZE,
                PINK,
            ); */
        }
    }

    draw_rectangle(400.0, 1300.0, 100.0, 200.0, MAGENTA);
}
