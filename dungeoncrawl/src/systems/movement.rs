use crate::{events::WantsToMove, prelude::*};

pub fn movement_system(
    //ecs: World,
    map: Res<Map>,
    mut camera: ResMut<Viewport>,
    mut commands: Commands,
    mut reader: EventReader<WantsToMove>,
    player: Query<Entity, With<Player>>,
) {
    for event in reader.read() {
        if map.can_enter_tile(event.destination) {
            commands.entity(event.entity).insert(event.destination);

            if player.get(event.entity).is_ok() {
                camera.on_player_move(event.destination)
            }
        }
    }
}
