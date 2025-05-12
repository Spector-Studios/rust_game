use crate::prelude::*;

pub fn map_render_system(map: Res<Map>, viewport: Res<Viewport>, sprite_sheet: Res<SpriteSheet>) {
    for y in viewport.view_area.y1..=viewport.view_area.y2 {
        let screen_y = viewport.get_screen_y(y);
        for x in viewport.view_area.x1..=viewport.view_area.x2 {
            let screen_x = viewport.get_screen_x(x);
            let pt = TilePoint::new(x, y);

            if map.in_bounds(pt) {
                let idx = map_idx(pt.x, pt.y);
                //let screen_pos = camera.get_screen_pos(pt);

                match map.tiles[idx] {
                    TileType::Wall => {
                        draw_texture_ex(
                            &sprite_sheet.sprites,
                            screen_x,
                            screen_y,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                                source: Some(Rect::new(64.0, 96.0, 32.0, 32.0)),
                                ..Default::default()
                            },
                        );
                        //draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, RED);
                    }
                    TileType::Floor => draw_texture_ex(
                        &sprite_sheet.sprites,
                        screen_x,
                        screen_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                            source: Some(Rect::new(96.0, 0.0, 32.0, 32.0)),
                            ..Default::default()
                        },
                    ),
                }
            }
        }
    }
}
