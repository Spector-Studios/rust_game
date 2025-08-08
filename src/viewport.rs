use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct Viewport {
    pub view_area: TileRect,
}

impl Viewport {
    pub fn new(player_position: TilePoint) -> Self {
        let mut x = player_position.x - VIEWPORT_WIDTH_T / 2;
        let mut y = player_position.y - VIEWPORT_HEIGHT_T / 2;

        x = x.clamp(0, TILE_MAP_WIDTH - VIEWPORT_WIDTH_T);
        y = y.clamp(0, TILE_MAP_HEIGHT - VIEWPORT_HEIGHT_T);

        Self {
            view_area: TileRect::with_size(x, y, VIEWPORT_WIDTH_T, VIEWPORT_HEIGHT_T),
        }
    }

    pub fn viewport_centre() -> Vec2 {
        vec2(
            Self::x_offset() + VIEWPORT_WIDTH / 2.0,
            Self::y_offset() + VIEWPORT_HEIGHT / 2.0,
        )
    }

    #[inline]
    pub fn x_offset() -> f32 {
        (screen_width() - VIEWPORT_WIDTH) / 2.0
    }

    #[inline]
    pub fn y_offset() -> f32 {
        ((screen_height() - VIEWPORT_HEIGHT) / 2.0) * 0.7
    }

    #[inline]
    pub fn get_hud_screen_x(tile_x: i32) -> f32 {
        tile_x as f32 * TILE_SIZE + Self::x_offset()
    }

    #[inline]
    pub fn get_hud_screen_y(tile_y: i32) -> f32 {
        tile_y as f32 * TILE_SIZE + Self::y_offset()
    }

    #[inline]
    pub fn get_screen_x(&self, tile_x: i32) -> f32 {
        (tile_x - self.view_area.x1) as f32 * TILE_SIZE + Self::x_offset()
    }

    #[inline]
    pub fn get_screen_y(&self, tile_y: i32) -> f32 {
        (tile_y - self.view_area.y1) as f32 * TILE_SIZE + Self::y_offset()
    }

    pub fn on_player_move(&mut self, player_position: TilePoint) {
        let mut x = player_position.x - VIEWPORT_WIDTH_T / 2;
        let mut y = player_position.y - VIEWPORT_HEIGHT_T / 2;

        x = x.clamp(0, TILE_MAP_WIDTH - VIEWPORT_WIDTH_T - 2);
        y = y.clamp(0, TILE_MAP_HEIGHT - VIEWPORT_HEIGHT_T - 2);

        self.view_area = TileRect::with_size(x, y, VIEWPORT_WIDTH_T, VIEWPORT_HEIGHT_T);
    }
}
