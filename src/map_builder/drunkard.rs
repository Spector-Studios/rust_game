use bracket_pathfinding::prelude::{Algorithm2D, DijkstraMap};

use crate::{map_builder::MapArchitect, prelude::*};

const STAGGER_DISTANCE: usize = 500;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl DrunkardsWalkArchitect {
    fn drunkard(&self, start: TilePoint, map: &mut Map) {
        let mut drunkard_pos = start;
        let mut distance_staggered = 0;

        loop {
            let drunkard_idx = map.point2d_to_index(drunkard_pos.into());
            map.tiles[drunkard_idx] = TileType::Floor;

            match gen_range(0, 4) {
                0 => drunkard_pos.x += 1,
                1 => drunkard_pos.x -= 1,
                2 => drunkard_pos.y += 1,
                _ => drunkard_pos.y -= 1,
            }

            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::empty();

        mb.fill(TileType::Wall);
        let center = TilePoint::new(TILE_MAP_WIDTH / 2, TILE_MAP_HEIGHT / 2);
        self.drunkard(center, &mut mb.map);

        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            self.drunkard(
                TilePoint::new(gen_range(0, TILE_MAP_WIDTH), gen_range(0, TILE_MAP_HEIGHT)),
                &mut mb.map,
            );

            let dijkstra_map = DijkstraMap::new(
                TILE_MAP_WIDTH,
                TILE_MAP_HEIGHT,
                &[mb.map.point2d_to_index(center.into())],
                &mb.map,
                1024.0,
            );

            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }

        mb.monster_spawns = mb.spawn_monsters(center);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant().into();

        mb
    }
}
