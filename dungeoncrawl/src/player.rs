use crate::prelude::*;

pub struct Player {
    pub position: TilePoint,
    frame_time: f32,
}

impl Player {
    pub fn new(position: TilePoint) -> Self {
        Self {
            position,
            frame_time: 0.0,
        }
    }

    /* pub fn render(&self, area: &TileRect) {
        draw_rectangle(
            self.position.to_screen_pos(area).x + 10.0,
            self.position.to_screen_pos(area).y + 10.0,
            30.0,
            30.0,
            BLUE,
        );

        #[cfg(debug_assertions)]
        draw_text(
            format!("{}, {}", self.position.x, self.position.y).as_str(),
            0.0,
            20.0,
            30.0,
            BLACK,
        );
    } */

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
