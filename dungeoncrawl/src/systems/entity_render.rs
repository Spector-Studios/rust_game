use crate::camera::Camera;
use crate::prelude::*;

pub fn entity_render_system(
    camera: Res<Camera>,
    sprite_sheet: Res<SpriteSheet>,
    entity_query: Query<(&TilePoint, &Render)>,
) {
    for (pos, render) in entity_query.iter() {
        let sprite_source = match render.texture {
            EntityType::Player => Rect::new(96.0, 32.0, 32.0, 32.0),
            EntityType::Goblin => Rect::new(32.0, 32.0, 32.0, 32.0),
            EntityType::Giant => Rect::new(0.0, 32.0, 32.0, 32.0),
            EntityType::Twoheads => Rect::new(32.0, 96.0, 32.0, 32.0),
            EntityType::Warrior => Rect::new(96.0, 96.0, 32.0, 32.0),
        };

        if camera.view_area.contains(*pos) {
            draw_texture_ex(
                &sprite_sheet.sprites,
                camera.get_screen_x(pos.x),
                camera.get_screen_y(pos.y),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    source: Some(sprite_source),
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

    //draw_rectangle(400.0, 1300.0, 100.0, 200.0, MAGENTA);
}
