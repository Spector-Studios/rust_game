use crate::prelude::*;

pub fn entity_render_system(
    viewport: Res<Viewport>,
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

        if viewport.view_area.contains(*pos) {
            draw_texture_ex(
                &sprite_sheet.sprites,
                viewport.get_screen_x(pos.x),
                viewport.get_screen_y(pos.y),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    source: Some(sprite_source),
                    ..Default::default()
                },
            );
        }
    }
}
