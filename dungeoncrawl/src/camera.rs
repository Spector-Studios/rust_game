use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct Camera {
    pub view_area: TileRect,
}

impl Camera {
    pub fn new(player_position: TilePoint) -> Self {
        let mut x = player_position.x - VIEWPORT_WIDTH_T / 2;
        let mut y = player_position.y - VIEWPORT_HEIGHT_T / 2;

        x = x.clamp(0, TILE_MAP_WIDTH - VIEWPORT_WIDTH_T);
        y = y.clamp(0, TILE_MAP_HEIGHT - VIEWPORT_HEIGHT_T);

        Self {
            view_area: TileRect::with_size(x, y, VIEWPORT_WIDTH_T, VIEWPORT_HEIGHT_T),
        }
    }

    pub fn get_screen_x(&self, tile_x: i32) -> f32 {
        (tile_x - self.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X
    }

    pub fn get_screen_y(&self, tile_y: i32) -> f32 {
        (tile_y - self.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y
    }

    pub fn on_player_move(&mut self, player_position: TilePoint) {
        let mut x = player_position.x - VIEWPORT_WIDTH_T / 2;
        let mut y = player_position.y - VIEWPORT_HEIGHT_T / 2;

        x = x.clamp(0, TILE_MAP_WIDTH - VIEWPORT_WIDTH_T - 2);
        y = y.clamp(0, TILE_MAP_HEIGHT - VIEWPORT_HEIGHT_T - 2);

        self.view_area = TileRect::with_size(x, y, VIEWPORT_WIDTH_T, VIEWPORT_HEIGHT_T);
    }
}
