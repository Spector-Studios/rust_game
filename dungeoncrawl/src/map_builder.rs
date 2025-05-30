use bracket_pathfinding::prelude::{Algorithm2D, DijkstraMap};

use crate::prelude::*;

const NUM_ROOMS: usize = 20;
const PADDING: i32 = 4;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<TileRect>,
    pub player_start: TilePoint,
    pub amulet_start: TilePoint,
}

impl MapBuilder {
    pub fn new(rng: &mut Rng) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: TilePoint::zero(),
            amulet_start: TilePoint::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridor();
        mb.player_start = mb.rooms[0].centre();

        let dijkstra_map = DijkstraMap::new(
            TILE_MAP_WIDTH,
            TILE_MAP_HEIGHT,
            &[mb.map.point2d_to_index(mb.player_start.into())],
            &mb.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;

        mb.amulet_start = mb
            .map
            .index_to_point2d(
                dijkstra_map
                    .map
                    .iter()
                    .enumerate()
                    .filter(|(_, dist)| *dist < UNREACHABLE)
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .unwrap()
                    .0,
            )
            .into();

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
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
