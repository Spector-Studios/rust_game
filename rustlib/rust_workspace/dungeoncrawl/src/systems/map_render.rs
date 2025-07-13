use crate::{prelude::*, resources::Theme};

pub fn map_render_system(
    map: Res<Map>,
    viewport: Res<Viewport>,
    sprite_sheet: Res<SpriteSheet>,
    theme: Res<Theme>,
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

                // TODO Make Rng a resource
                let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
                let source = Some(theme.tile_to_render(map.tiles[idx], &mut rng));
                draw_texture_ex(
                    theme.texture(&sprite_sheet),
                    screen_x,
                    screen_y,
                    tint,
                    DrawTextureParams {
                        dest_size: DEST_SIZE,
                        source,
                        ..Default::default()
                    },
                );
            }
        }
    }
}
