use bracket_pathfinding::prelude::{Algorithm2D, DijkstraMap, DistanceAlg};

use crate::{
    events::{WantsToAttack, WantsToMove},
    prelude::*,
    resources::PathfindingMap,
};

pub fn chasing_system(
    map: Res<Map>,
    pathfinding_map: Res<PathfindingMap>,
    chasers: Query<(Entity, &TilePoint, &FieldOfView), With<ChasePlayer>>,
    creatures_query: Query<(Entity, &TilePoint, &Health, Option<&Player>)>,
    mut attack_writer: EventWriter<WantsToAttack>,
    mut move_writer: EventWriter<WantsToMove>,
) {
    if pathfinding_map.is_stale {
        return;
    }

    let dijkstra_map = &pathfinding_map.dijsktra_map;
    let (_, player_pos, _, _) = creatures_query
        .iter()
        .find(|(_, _, _, option_player)| option_player.is_some())
        .unwrap();

    chasers
        .iter()
        .filter(|(_, _, fov)| fov.visible_tiles.contains(&(*player_pos).into()))
        .for_each(|(attacker, pos, _)| {
            let idx = map.point2d_to_index((*pos).into());

            if let Some(destination) = DijkstraMap::find_lowest_exit(dijkstra_map, idx, &*map) {
                let distance =
                    DistanceAlg::Pythagoras.distance2d((*pos).into(), (*player_pos).into());

                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination).into()
                } else {
                    *player_pos
                };

                let mut attacked = false;
                creatures_query
                    .iter()
                    .filter(|(_, target_pos, _, _)| **target_pos == destination)
                    .for_each(|(victim, _, _, option_player)| {
                        if option_player.is_some() {
                            attack_writer.write(WantsToAttack { attacker, victim });
                        }
                        attacked = true;
                    });

                if !attacked {
                    move_writer.write(WantsToMove {
                        entity: attacker,
                        destination,
                        is_player: false,
                    });
                }
            }
        });
}
