use crate::{camera::Camera, prelude::*};

pub struct Player {
    pub position: TilePoint,
    frame_time: f32,
    player_texture: Texture2D,
}

impl Player {
    pub fn new(position: TilePoint, player_texture: Texture2D) -> Self {
        Self {
            position,
            frame_time: 0.0,
            player_texture,
        }
    }

    pub fn render(&self, camera: &Camera) {
        /* draw_rectangle(
            (player.position.x - self.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X + 5.0,
            (player.position.y - self.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y + 5.0,
            40.0,
            40.0,
            BLUE,
        ); */
        draw_texture_ex(
            &self.player_texture,
            (self.position.x - camera.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X + 5.0,
            (self.position.y - camera.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y + 5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(50.0, 50.0)),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, input: ButtonState, map: &Map) {
        if self.frame_time < 8.0 {
            self.frame_time += get_frame_time() * 100.0;

            #[cfg(debug_assertions)]
            draw_rectangle(screen_width() - 50.0, 0.0, 50.0, 50.0, RED);

            return;
        }

        if input == ButtonState::default() {
            #[cfg(debug_assertions)]
            draw_rectangle(screen_width() - 50.0, 0.0, 50.0, 50.0, YELLOW);

            return;
        }

        //macroquad has "inverted" y axis
        let delta = TilePoint::new(input.dpad_x, -input.dpad_y);

        let new_position = self.position + delta;

        // TODO
        if map.can_enter_tile(new_position) {
            /* if (new_position.x - area.x < 1)
                || (new_position.y - area.y < 1)
                || (new_position.x >= area.x + area.w)
                || (new_position.y >= area.y + area.h)
            {
                if (new_position.x < 2 && delta.x < 0)
                    || (new_position.y < 2 && delta.y < 0)
                    || (new_position.x == TILE_MAP_WIDTH && delta.x > 0)
                    || (new_position.y == TILE_MAP_HEIGHT && delta.y > 0)
                {
                } else {
                    area.x += delta.x;
                    area.y += delta.y;
                }
            } */
            self.position = new_position;
        }

        #[cfg(debug_assertions)]
        draw_rectangle(screen_width() - 50.0, 0.0, 50.0, 50.0, BLUE);

        self.frame_time = 0.0;
    }
}
