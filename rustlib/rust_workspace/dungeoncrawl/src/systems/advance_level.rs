use std::collections::HashSet;

use bevy_state::state::NextState;
use bracket_pathfinding::prelude::Algorithm2D;

use crate::{TurnState, prelude::*};

pub fn advance_level(
    mut next_state: ResMut<NextState<TurnState>>,
    mut player: Query<(Entity, &mut FieldOfView, &mut TilePoint, &mut Player)>,
    carried: Query<(Entity, &Carried)>,
    all_entities: Query<Entity>,
    mut commands: Commands,
) {
    let (player_entity, mut player_fov, mut player_pos, mut player) = player.single_mut().unwrap();

    let mut entities_to_keep = HashSet::new();

    entities_to_keep.insert(player_entity);

    carried
        .iter()
        .filter(|(_, carried)| carried.0 == player_entity)
        .for_each(|(entity, _)| {
            entities_to_keep.insert(entity);
        });

    all_entities
        .iter()
        .filter(|e| !entities_to_keep.contains(e))
        .for_each(|e| commands.entity(e).despawn());

    let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as u64);
    let mut mb = MapBuilder::new(&mut rng);

    player_fov.is_stale = true;
    let curr_level = player.map_level;
    player.map_level += 1;
    *player_pos = mb.player_start;

    if curr_level == 2 {
        spawn_amulet(&mut commands, mb.amulet_start);
    } else {
        let end_idx = mb.map.point2d_to_index(mb.amulet_start.into());
        mb.map.tiles[end_idx] = TileType::Stair;
    }

    mb.monster_spawns
        .iter()
        .for_each(|pos| spawn_entity(&mut commands, *pos, &mut rng));

    commands.insert_resource(mb.map);
    commands.insert_resource(Viewport::new(mb.player_start));
    commands.insert_resource(mb.theme);

    next_state.set(TurnState::AwaitingInput);
}
