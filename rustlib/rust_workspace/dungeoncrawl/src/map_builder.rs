mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

use crate::{
    map_builder::{drunkard::DrunkardsWalkArchitect, prefab::apply_prefab},
    prelude::*,
    resources::Theme,
};
use automata::CellularAutomataArchitect;
use bracket_pathfinding::prelude::{Algorithm2D, DijkstraMap, DistanceAlg};
use rooms::RoomsArchitect;

trait MapArchitect {
    fn build(&mut self, rng: &mut Rng) -> MapBuilder;
}

pub trait MapTheme: Send + Sync {
    fn tile_to_render(&self, tile_type: TileType) -> Rect;
    fn texture<'a>(&self, sprite_sheet: &'a SpriteSheet) -> &'a Texture2D;
}

const NUM_ROOMS: usize = 10;
const PADDING: i32 = 4;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<TileRect>,
    pub player_start: TilePoint,
    pub amulet_start: TilePoint,
    pub monster_spawns: Vec<TilePoint>,
    pub theme: Theme,
}

impl MapBuilder {
    pub fn empty() -> Self {
        Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: TilePoint::zero(),
            amulet_start: TilePoint::zero(),
            monster_spawns: Vec::new(),
            theme: Theme {
                theme: themes::FortressTheme::boxed_new(),
            },
        }
    }
    pub fn new(rng: &mut Rng) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.i8(0..3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };
        let mut mb = architect.build(rng);
        apply_prefab(&mut mb, rng);

        mb.theme.theme = match rng.i8(0..2) {
            0 => {
                info!("Fortress Theme");
                themes::FortressTheme::boxed_new()
            }
            _ => {
                info!("Forest theme");
                themes::ForestTheme::boxed_new()
            }
        };

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> impl Into<TilePoint> {
        let dijkstra_map = DijkstraMap::new(
            TILE_MAP_WIDTH,
            TILE_MAP_HEIGHT,
            &[self.map.point2d_to_index(self.player_start.into())],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn spawn_monsters(&self, start: TilePoint, rng: &mut Rng) -> Vec<TilePoint> {
        const NUM_MONSTERS: usize = 50;

        let mut spawnable_tiles: Vec<TilePoint> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras
                        .distance2d(start.into(), self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx).into())
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_idx = rng.usize(0..spawnable_tiles.len());
            spawns.push(spawnable_tiles[target_idx]);
            spawnable_tiles.remove(target_idx);
        }

        spawns
    }

    fn build_random_rooms(&mut self, rng: &mut Rng) {
        while self.rooms.len() < NUM_ROOMS {
            let room = TileRect::with_size(
                rng.i32(1..TILE_MAP_WIDTH - 10),
                rng.i32(1..TILE_MAP_HEIGHT - 10),
                rng.i32(4..10),
                rng.i32(4..10),
            );

            let mut overlaps = false;
            let padded_room = TileRect::with_corners(
                room.x1 - PADDING,
                room.y1 - PADDING,
                room.x2 + PADDING,
                room.y2 + PADDING,
            );
            for r in self.rooms.iter() {
                if padded_room.intersects(r) {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                room.for_each(|point| {
                    if point.x > 0
                        && point.x < TILE_MAP_WIDTH
                        && point.y > 0
                        && point.y < TILE_MAP_HEIGHT
                    {
                        let idx = map_idx(point);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(TilePoint::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(TilePoint::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridor(&mut self) {
        self.rooms.sort_by(|a, b| a.centre().x.cmp(&b.centre().x));
        let rooms = self.rooms.clone();

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].centre();
            let new = room.centre();

            if rand::gen_range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
