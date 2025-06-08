use crate::{events::WantsToMove, prelude::*};

// TODO Make this smooth transition
pub fn movement_system(
    map: Res<Map>,
    mut camera: ResMut<Viewport>,
    mut commands: Commands,
    mut reader: EventReader<WantsToMove>,
    mut fov: Query<&mut FieldOfView>,
) {
    for event in reader.read() {
        if map.can_enter_tile(event.destination) {
            commands.entity(event.entity).insert(event.destination);
            if let Ok(fov) = fov.get_mut(event.entity) {
                commands.entity(event.entity).insert(fov.clone_stale());
            }

            if event.is_player {
                camera.on_player_move(event.destination)
            }
        }
    }
}
