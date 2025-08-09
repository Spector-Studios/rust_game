use bracket_pathfinding::prelude::{Algorithm2D, DistanceAlg};

use crate::prelude::*;

use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn random_noise_map(&self, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = gen_range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbours = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0)
                    && map.tiles[map_idx(TilePoint::new(x + ix, y + iy))] == TileType::Wall
                {
                    neighbours += 1;
                }
            }
        }

        neighbours
    }

    fn iteration(&self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1..TILE_MAP_HEIGHT - 1 {
            for x in 1..TILE_MAP_WIDTH - 1 {
                let neighbours = self.count_neighbours(x, y, map);
                let idx = map_idx(TilePoint::new(x, y));
                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }

        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> TilePoint {
        let center = TilePoint::new(TILE_MAP_WIDTH / 2, TILE_MAP_HEIGHT / 2);

        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center.into(), map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point).into()
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::empty();

        self.random_noise_map(&mut mb.map);

        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }

        mb.player_start = self.find_start(&mb.map);
        mb.amulet_start = mb.find_most_distant().into();
        mb.monster_spawns = mb.spawn_monsters(mb.player_start);

        mb
    }
}
