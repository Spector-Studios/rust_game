use crate::prelude::*;

pub fn collisions_system(
    mut commands: Commands,
    player_query: Query<&TilePoint, With<Player>>,
    enemy_query: Query<(Entity, &TilePoint), With<Enemy>>,
) {
    for player_pos in player_query.iter() {
        enemy_query
            .iter()
            .filter(|(_, enemy_pos)| *enemy_pos == player_pos)
            .for_each(|(enemy_entiry, _)| commands.entity(enemy_entiry).despawn());
    }
}
