use crate::{events::WantsToAttack, prelude::*};

pub fn combat_system(
    mut attack_reader: EventReader<WantsToAttack>,
    mut commands: Commands,
    mut health_query: Query<&mut Health>,
) {
    for WantsToAttack { attacker, victim } in attack_reader.read() {
        if let Ok(mut health) = health_query.get_mut(*victim) {
            debug!("Health before attack: {}", health.current);

            health.current -= 1;

            debug!("Health after: {}", health.current);

            if health.current < 1 {
                commands.entity(*victim).despawn();
            }
        }
    }
}
