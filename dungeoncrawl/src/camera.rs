use crate::prelude::*;

pub struct Camera {
    view_area: TileRect,
    player_texture: Texture2D,
}

impl Camera {
    pub fn new(player_texture: Texture2D) -> Self {
        Self {
            view_area: TileRect::with_size(0, 0, VIEWPORT_WIDTH_T, VIEWPORT_HEIGHT_T),
            player_texture,
        }
    }

    pub fn update(&mut self, player: &Player) {
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
    }

    pub fn render(&self, map: &Map, player: &Player) {
        let mut screen_x;
        let mut screen_y;
        // map
        for y in self.view_area.y1..self.view_area.y2 {
            screen_y = (y - self.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y;
            for x in self.view_area.x1..self.view_area.x2 {
                let idx = (y * TILE_MAP_WIDTH + x) as usize;
                screen_x = (x - self.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X;

                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, SKYBLUE)
                    }
                    TileType::Wall => {
                        draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, GREEN)
                    }
                    TileType::FRoom => {
                        draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, PINK)
                    }
                    TileType::FCorridor => {
                        draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, VIOLET)
                    }
                }

                #[cfg(debug_assertions)]
                {
                    draw_rectangle_lines(screen_x, screen_y, TILE_SIZE, TILE_SIZE, 2.0, BLACK);
                    draw_text(
                        format!("{}", x).as_str(),
                        screen_x + 5.0,
                        screen_y + 15.0,
                        30.0,
                        BLACK,
                    );
                    draw_text(
                        format!("{}", y).as_str(),
                        screen_x + 20.0,
                        screen_y + 45.0,
                        30.0,
                        BLACK,
                    );
                }
            }
        }

        // player
        draw_rectangle(
            (player.position.x - self.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X + 5.0,
            (player.position.y - self.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y + 5.0,
            40.0,
            40.0,
            BLUE,
        );
        draw_texture_ex(
            &self.player_texture,
            (player.position.x - self.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X + 5.0,
            (player.position.y - self.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y + 5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(50.0, 50.0)),
                ..Default::default()
            },
        );
        #[cfg(debug_assertions)]
        draw_text(
            format!("{}, {}", player.position.x, player.position.y).as_str(),
            0.0,
            20.0,
            30.0,
            WHITE,
        );

        // debug
        #[cfg(debug_assertions)]
        {
            draw_text(format!("{}", get_fps()).as_str(), 0.0, 50.0, 30.0, WHITE);
            draw_text(
                format!("{}", 1.0 / get_frame_time()).as_str(),
                0.0,
                80.0,
                30.0,
                WHITE,
            );
        }
    }
}
