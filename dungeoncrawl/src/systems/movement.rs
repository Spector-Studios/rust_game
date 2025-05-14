use crate::{events::WantsToMove, prelude::*};

pub fn movement_system(
    //ecs: World,
    map: Res<Map>,
    mut camera: ResMut<Viewport>,
    mut commands: Commands,
    mut reader: EventReader<WantsToMove>,
) {
    for event in reader.read() {
        if map.can_enter_tile(event.destination) {
            commands.entity(event.entity).insert(event.destination);

            if event.is_player {
                camera.on_player_move(event.destination)
            }
        }
    }
}
