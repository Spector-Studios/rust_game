use crate::prelude::*;
use bracket_pathfinding::prelude::{Algorithm2D, DistanceAlg, Point, SmallVec};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Stair,
}

pub fn map_idx<T: Into<TilePoint>>(pos: T) -> usize {
    let pos: TilePoint = pos.into();

    (pos.y * TILE_MAP_WIDTH + pos.x) as usize
}

#[derive(Resource, Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn in_bounds<T: Into<TilePoint>>(&self, point: T) -> bool {
        let point = point.into();
        (point.x >= 0)
            && (point.y >= 0)
            && (point.x < TILE_MAP_WIDTH)
            && (point.y < TILE_MAP_HEIGHT)
    }

    pub fn can_enter_tile(&self, point: TilePoint) -> bool {
        if !self.in_bounds(point) {
            return false;
        }
        let tile_type = self.tiles[map_idx(point)];
        tile_type == TileType::Floor || tile_type == TileType::Stair
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

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
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
