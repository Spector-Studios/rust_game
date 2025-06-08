use crate::prelude::*;

pub fn map_render_system(
    map: Res<Map>,
    viewport: Res<Viewport>,
    sprite_sheet: Res<SpriteSheet>,
    fov: Query<&FieldOfView, With<Player>>,
) {
    let player_fov = fov.single().unwrap();

    for y in viewport.view_area.y1..=viewport.view_area.y2 {
        let screen_y = viewport.get_screen_y(y);
        for x in viewport.view_area.x1..=viewport.view_area.x2 {
            let screen_x = viewport.get_screen_x(x);
            let pt = TilePoint::new(x, y);
            let idx = map_idx(pt);

            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt.into()) | map.revealed_tiles[idx])
            {
                let tint = if player_fov.visible_tiles.contains(&pt.into()) {
                    WHITE
                } else {
                    GRAY
                };

                match map.tiles[idx] {
                    TileType::Wall => draw_texture_ex(
                        sprite_sheet.sprites.get(&SpriteKey::Wall).unwrap(),
                        screen_x,
                        screen_y,
                        tint,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                    ),

                    TileType::Floor => draw_texture_ex(
                        sprite_sheet.sprites.get(&SpriteKey::Floor).unwrap(),
                        screen_x,
                        screen_y,
                        tint,
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
