use std::collections::VecDeque;

use bracket_pathfinding::prelude::DijkstraMap;

use crate::prelude::*;

#[derive(Resource)]
pub struct PathfindingMap {
    pub dijsktra_map: DijkstraMap,
    pub is_stale: bool,
}

impl PathfindingMap {
    pub fn new(search_targets: &[usize], map: &Map) -> Self {
        Self {
            dijsktra_map: DijkstraMap::new(
                TILE_MAP_WIDTH,
                TILE_MAP_HEIGHT,
                search_targets,
                map,
                1024.0,
            ),
            is_stale: false,
        }
    }
}

#[derive(Resource, Debug)]
pub struct EnemyQueue(pub VecDeque<Entity>);
