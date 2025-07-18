use bracket_pathfinding::prelude::{Algorithm2D, DijkstraMap, DistanceAlg};

use crate::prelude::*;

/// Updates all entities with the `ChasePlayer` component to move toward the player using Dijkstra pathfinding.
///
/// For each chaser entity, determines the optimal adjacent tile to approach the player and inserts a movement animation toward that destination. If the chaser is within close proximity to the player, it targets the player's exact position. Panics if there is not exactly one player entity.
///
/// # Examples
///
/// ```
/// // System usage in Bevy schedule:
/// app.add_system(chasing_system);
/// ```
pub fn chasing_system(
    mut commands: Commands,
    map: Res<Map>,
    chasers: Query<(Entity, &TilePoint), With<ChasePlayer>>,
    player: Query<&TilePoint, With<Player>>,
) {
    let player_pos = player.single().expect("More than one or no players");
    let player_idx = map.point2d_to_index((*player_pos).into());

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        TILE_MAP_WIDTH,
        TILE_MAP_HEIGHT,
        &search_targets,
        &*map,
        1024.0,
    );

    chasers.iter().for_each(|(attacker, pos)| {
        let idx = map.point2d_to_index((*pos).into());

        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, &*map) {
            let distance = DistanceAlg::Pythagoras.distance2d((*pos).into(), (*player_pos).into());

            let destination = if distance > 1.2 {
                map.index_to_point2d(destination).into()
            } else {
                *player_pos
            };

            commands
                .entity(attacker)
                .insert(Animation::new_movement(*pos, destination));
        }
    });
}
