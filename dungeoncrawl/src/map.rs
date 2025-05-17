use crate::prelude::*;
use bracket_pathfinding::prelude::{Algorithm2D, DistanceAlg, Point, SmallVec};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    // FRoom,
    // FCorridor,
}

pub fn map_idx<T: Into<TilePoint>>(pos: T) -> usize {
    let pos: TilePoint = pos.into();

    (pos.y * TILE_MAP_WIDTH + pos.x) as usize
}

#[derive(Resource, Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    //pub floor_texture: Texture2D,
    //pub wall_texture: Texture2D,
    //pub displayed_corner_tile: TilePoint,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            //floor_texture,
            //wall_texture,
            //displayed_corner_tile: TilePoint::new(3, 3),
        }
    }

    // pub fn render(&self, camera: &Camera) {
    //     let mut screen_x;
    //     let mut screen_y;
    //     // map
    //     for y in camera.view_area.y1..camera.view_area.y2 {
    //         screen_y = (y - camera.view_area.y1) as f32 * TILE_SIZE + VIEWPORT_Y;
    //         for x in camera.view_area.x1..camera.view_area.x2 {
    //             let idx = (y * TILE_MAP_WIDTH + x) as usize;
    //             screen_x = (x - camera.view_area.x1) as f32 * TILE_SIZE + VIEWPORT_X;

    //             match self.tiles[idx] {
    //                 TileType::Floor => {
    //                     draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, SKYBLUE)
    //                 }
    //                 TileType::Wall => {
    //                     draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, GREEN)
    //                 }
    //             }

    //             #[cfg(debug_assertions)]
    //             {
    //                 draw_rectangle_lines(screen_x, screen_y, TILE_SIZE, TILE_SIZE, 2.0, BLACK);
    //                 draw_text(
    //                     format!("{}", x).as_str(),
    //                     screen_x + 5.0,
    //                     screen_y + 15.0,
    //                     30.0,
    //                     BLACK,
    //                 );
    //                 draw_text(
    //                     format!("{}", y).as_str(),
    //                     screen_x + 20.0,
    //                     screen_y + 45.0,
    //                     30.0,
    //                     BLACK,
    //                 );
    //             }
    //         }
    //     }
    // }
    pub fn in_bounds<T: Into<TilePoint>>(&self, point: T) -> bool {
        let point = point.into();
        (point.x >= 0)
            && (point.y >= 0)
            && (point.x < TILE_MAP_WIDTH)
            && (point.y < TILE_MAP_HEIGHT)
    }

    pub fn can_enter_tile(&self, point: TilePoint) -> bool {
        self.in_bounds(point) && (self.tiles[map_idx(point)] == TileType::Floor)
    }

    pub fn try_idx(&self, point: TilePoint) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point))
        }
    }

    fn valid_exit<T: Into<TilePoint>>(&self, loc: T, delta: T) -> Option<usize> {
        let loc: TilePoint = loc.into();
        let delta: TilePoint = delta.into();

        let destination = loc + delta;

        if self.in_bounds(destination) && self.can_enter_tile(destination) {
            let idx = self.point2d_to_index(destination.into());
            Some(idx)
        } else {
            None
        }
    }
}

impl bracket_pathfinding::prelude::BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        for delta in DIRECTIONS {
            if let Some(idx) = self.valid_exit(location, delta.into()) {
                exits.push((idx, 1.0));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl bracket_pathfinding::prelude::Algorithm2D for Map {
    fn dimensions(&self) -> bracket_pathfinding::prelude::Point {
        Point::new(TILE_MAP_WIDTH, TILE_MAP_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}
