use bracket_pathfinding::prelude::{Algorithm2D, DijkstraMap, DistanceAlg};

use crate::{
    events::{WantsToAttack, WantsToMove},
    prelude::*,
};

pub fn chasing_system(
    map: Res<Map>,
    chasers: Query<(Entity, &TilePoint), With<ChasePlayer>>,
    creatures_query: Query<(Entity, &TilePoint, &Health, Option<&Player>)>,
    player: Query<&TilePoint, With<Player>>,
    mut attack_writer: EventWriter<WantsToAttack>,
    mut move_writer: EventWriter<WantsToMove>,
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
