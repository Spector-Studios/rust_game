use crate::prelude::*;

pub fn player_collision_system(
    mut commands: Commands,
    player_query: Query<&TilePoint, With<Player>>,
    enemy_query: Query<(Entity, &TilePoint), With<Enemy>>,
) {
    let player_pos = *player_query
        .single_inner()
        .expect("More than one or no players present.");

    enemy_query
        .iter()
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(enemy_entity, _)| commands.entity(enemy_entity).despawn());
}
