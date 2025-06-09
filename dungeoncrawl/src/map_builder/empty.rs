use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn build(&mut self, rng: &mut Rng) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: TilePoint::zero(),
            amulet_start: TilePoint::zero(),
            monster_spawns: Vec::new(),
        };

        mb.fill(TileType::Floor);
        mb.player_start = TilePoint::new(VIEWPORT_WIDTH_T / 2, VIEWPORT_HEIGHT_T / 2);
        mb.amulet_start = mb.find_most_distant().into();

        for _ in 0..50 {
            mb.monster_spawns.push(TilePoint::new(
                rng.i32(1..TILE_MAP_WIDTH),
                rng.i32(1..TILE_MAP_HEIGHT),
            ));
        }

        mb
    }
}
