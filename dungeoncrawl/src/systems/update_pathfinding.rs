use crate::{prelude::*, resources::PathfindingMap};
use bracket_pathfinding::prelude::Algorithm2D;

pub fn update_pathfinding(
    map: Res<Map>,
    mut pathfinding_map: ResMut<PathfindingMap>,
    player_pos_query: Query<&TilePoint, With<Player>>,
) {
    let player_pos = player_pos_query
        .single()
        .expect("More than one or no players");
    let player_idx = map.point2d_to_index((*player_pos).into());

    let search_targets = vec![player_idx];
    if pathfinding_map.is_stale {
        *pathfinding_map = PathfindingMap::new(&search_targets, &map);
    }
}
