use crate::prelude::*;

#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] texture_store: &TextureStore,
) {
    // let mut render_target = render_target(VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
    // let offset = TilePoint::new(camera.view_area.x1, camera.view_area.y1);

    let mut render_camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT));
    render_camera.render_target = Some(texture_store.map_render.clone());
    //set_camera(&render_camera);

    for y in camera.view_area.y1..=camera.view_area.y2 {
        for x in camera.view_area.x1..=camera.view_area.x2 {
            let pt = TilePoint::new(x, y);

            if map.in_bounds(pt) {
                let idx = map_idx(pt.x, pt.y);
                let screen_pos = camera.get_screen_pos(pt);
                match map.tiles[idx] {
                    TileType::Wall => {
                        draw_rectangle(screen_pos.x, screen_pos.y, TILE_SIZE, TILE_SIZE, RED)
                    }
                    TileType::Floor => {
                        draw_rectangle(screen_pos.x, screen_pos.y, TILE_SIZE, TILE_SIZE, GREEN)
                    }
                }
            }
        }
    }
}
