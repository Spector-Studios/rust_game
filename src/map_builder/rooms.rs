use super::MapArchitect;
use crate::map_builder::themes::FortressTheme;
use crate::prelude::*;
use crate::resources::Theme;
use bracket_pathfinding::prelude::Algorithm2D;
use bracket_pathfinding::prelude::DijkstraMap;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: TilePoint::zero(),
            amulet_start: TilePoint::zero(),
            monster_spawns: Vec::new(),
            theme: Theme {
                theme: FortressTheme::boxed_new(),
            },
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms();
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

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.centre());
        }

        mb
    }
}
