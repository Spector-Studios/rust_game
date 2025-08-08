use crate::{events::ActivateItem, prelude::*};

pub fn use_item(
    mut map: ResMut<Map>,
    mut event_reader: EventReader<ActivateItem>,
    mut commands: Commands,
    healing_query: Query<&ProvidesHealing, With<Carried>>,
    map_query: Query<&ProvidesDungeonMap, With<Carried>>,
    mut health_query: Query<&mut Health>,
) {
    for ActivateItem { used_by, item } in event_reader.read() {
        if let Ok(healing) = healing_query.get(*item) {
            if let Ok(mut health) = health_query.get_mut(*used_by) {
                health.current = i32::min(health.max, health.current + healing.amount);
            }
        }

        if let Ok(_mapping) = map_query.get(*item) {
            map.revealed_tiles.iter_mut().for_each(|t| *t = true);
        }

        commands.entity(*item).despawn();
    }
}
