use crate::prelude::*;

pub fn map_render_system(map: Res<Map>, viewport: Res<Viewport>, sprite_sheet: Res<SpriteSheet>) {
    for y in viewport.view_area.y1..=viewport.view_area.y2 {
        let screen_y = viewport.get_screen_y(y);
        for x in viewport.view_area.x1..=viewport.view_area.x2 {
            let screen_x = viewport.get_screen_x(x);
            let pt = TilePoint::new(x, y);

            if map.in_bounds(pt) {
                let idx = map_idx(pt);

                match map.tiles[idx] {
                    TileType::Wall => {
                        draw_texture_ex(
                            sprite_sheet.sprites.get(&SpriteKey::Wall).unwrap(),
                            screen_x,
                            screen_y,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                                ..Default::default()
                            },
                        );
                    }

                    TileType::Floor => draw_texture_ex(
                        sprite_sheet.sprites.get(&SpriteKey::Floor).unwrap(),
                        screen_x,
                        screen_y,
                        WHITE,
                        // TODO store this somewhere (a CONST), It is used is all the rendering
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                    ),
                }
            }
        }
    }
}
