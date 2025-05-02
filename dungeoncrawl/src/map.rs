use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    FRoom,
    FCorridor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * TILE_MAP_WIDTH + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
    //pub displayed_corner_tile: TilePoint,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            //displayed_corner_tile: TilePoint::new(3, 3),
        }
    }

    /*
    pub fn render(&self, area: &TileRect) {
        for y in area.y1..area.y2 {
            for x in area.x1..area.x2 {
                let idx = map_idx(x, y);
                let screen_pos = TilePoint::new(x, y).to_screen_pos(area);

                match self.tiles[idx] {
                    TileType::Wall => {
                        draw_rectangle(screen_pos.x, screen_pos.y, TILE_SIZE, TILE_SIZE, GREEN)
                    }
                    TileType::Floor => {
                        draw_rectangle(screen_pos.x, screen_pos.y, TILE_SIZE, TILE_SIZE, RED);

                        #[cfg(debug_assertions)]
                        draw_text(
                            idx.to_string().as_str(),
                            screen_pos.x + 5.0,
                            screen_pos.y + TILE_SIZE / 2.0 + 10.0,
                            TILE_SIZE - 20.0,
                            BLACK,
                        );
                    }
                }

                #[cfg(debug_assertions)]
                draw_rectangle_lines(screen_pos.x, screen_pos.y, TILE_SIZE, TILE_SIZE, 2.0, BLACK);
            }
        }
    }
    */

    pub fn in_bounds(&self, point: TilePoint) -> bool {
        (point.x >= 0)
            && (point.y >= 0)
            && (point.x < TILE_MAP_WIDTH)
            && (point.y < TILE_MAP_HEIGHT)
    }

    pub fn can_enter_tile(&self, point: TilePoint) -> bool {
        self.in_bounds(point)
            && (self.tiles[map_idx(point.x, point.y)] == TileType::Floor
                || self.tiles[map_idx(point.x, point.y)] == TileType::FRoom
                || self.tiles[map_idx(point.x, point.y)] == TileType::FCorridor)
    }

    pub fn try_idx(&self, point: TilePoint) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
