use bracket_pathfinding::prelude::{Algorithm2D, DijkstraMap};

use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#-M--#---
---#----#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut Rng) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        TILE_MAP_WIDTH,
        TILE_MAP_WIDTH,
        &[mb.map.point2d_to_index(mb.player_start.into())],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = TileRect::with_size(
            rng.i32(0..TILE_MAP_WIDTH - FORTRESS.1),
            rng.i32(0..TILE_MAP_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt.into());
            let distance = dijkstra_map.map[idx];
            if distance > 20.0 && distance < 2000.0 && mb.amulet_start != pt {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(TilePoint::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
        }

        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .0
            .chars()
            .filter(|ch| *ch != '\n' && *ch != '\r')
            .collect();

        let mut i = 0;
        for y in placement.y..placement.y + FORTRESS.2 {
            for x in placement.x..placement.x + FORTRESS.1 {
                let pt = TilePoint::new(x, y);
                let idx = map_idx(pt);
                let ch = string_vec[i];
                match ch {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(pt);
                    }
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    _ => warn!("No idea! {}", ch),
                }
                i += 1;
            }
        }

        info!("Prefab placed at: {:?}", placement);
    } else {
        info!("Prefab not placed");
    }
}
