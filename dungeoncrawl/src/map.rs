use crate::{camera::Camera, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    // FRoom,
    // FCorridor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * TILE_MAP_WIDTH + x) as usize
}

#[derive(Resource, Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub floor_texture: Rect,
    pub wall_texture: Rect,
    //pub displayed_corner_tile: TilePoint,
}

impl Map {
    pub fn new(floor_texture: Rect, wall_texture: Rect) -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            floor_texture,
            wall_texture,
            //displayed_corner_tile: TilePoint::new(3, 3),
        }
    }

    pub fn render(&self, camera: &Camera) {
        let mut screen_x;
        let mut screen_y;
        // map
        for y in camera.view_area.y1..camera.view_area.y2 {
            screen_y = (y - camera.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y;
            for x in camera.view_area.x1..camera.view_area.x2 {
                let idx = (y * TILE_MAP_WIDTH + x) as usize;
                screen_x = (x - camera.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X;

                match self.tiles[idx] {
                    TileType::Floor => {
                        draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, SKYBLUE)
                    }
                    TileType::Wall => {
                        draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, GREEN)
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
    }
    pub fn in_bounds(&self, point: TilePoint) -> bool {
        (point.x >= 0)
            && (point.y >= 0)
            && (point.x < TILE_MAP_WIDTH)
            && (point.y < TILE_MAP_HEIGHT)
    }

    pub fn can_enter_tile(&self, point: TilePoint) -> bool {
        self.in_bounds(point) && (self.tiles[map_idx(point.x, point.y)] == TileType::Floor)
    }

    pub fn try_idx(&self, point: TilePoint) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
