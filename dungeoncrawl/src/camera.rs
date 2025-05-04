use crate::prelude::*;

pub struct Camera {
    pub view_area: TileRect,
    //player_texture: Texture2D,
    //floor_texture: Texture2D,
}

impl Camera {
    pub fn new(player_position: TilePoint) -> Self {
        Self {
            view_area: TileRect::with_size(
                player_position.x - VIEWPORT_WIDTH_T / 2,
                player_position.y - VIEWPORT_HEIGHT_T / 2,
                VIEWPORT_WIDTH_T,
                VIEWPORT_HEIGHT_T,
            ),
            //player_texture,
            //floor_texture,
        }
    }

    pub fn get_screen_pos(&self, tile_pos: TilePoint) -> Vec2 {
        vec2(
            (tile_pos.x - self.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X,
            (tile_pos.y - self.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y,
        )
    }

    /* pub fn update(&mut self, player: &Player) {
        if (player.position.x <= self.view_area.x1) && self.view_area.x1 != 0 {
            self.view_area.shift(D_LEFT);
        } else if (player.position.x >= self.view_area.x2 - 1)
            && self.view_area.x1 != TILE_MAP_WIDTH - VIEWPORT_WIDTH_T
        {
            self.view_area.shift(D_RIGHT);
        }

        if (player.position.y <= self.view_area.y1) && self.view_area.y1 != 0 {
            self.view_area.shift(D_DOWN);
        } else if (player.position.y >= self.view_area.y2 - 1)
            && self.view_area.y1 != TILE_MAP_HEIGHT - VIEWPORT_HEIGHT_T
        {
            self.view_area.shift(D_UP);
        }
    } */

    pub fn on_player_move(&mut self, player_position: TilePoint) {
        self.view_area.x1 = player_position.x - VIEWPORT_WIDTH_T / 2;
        self.view_area.x2 = player_position.x + VIEWPORT_WIDTH_T / 2;

        self.view_area.y1 = player_position.y - VIEWPORT_HEIGHT_T / 2;
        self.view_area.y2 = player_position.y + VIEWPORT_HEIGHT_T / 2;
    }
}
