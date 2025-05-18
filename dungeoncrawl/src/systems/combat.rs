use crate::{events::WantsToAttack, prelude::*};

// TODO Add some visual indicator that entity attacked
pub fn combat_system(
    mut attack_reader: EventReader<WantsToAttack>,
    mut commands: Commands,
    mut health_query: Query<&mut Health>,
    player_query: Query<&Player>,
) {
    for WantsToAttack { attacker, victim } in attack_reader.read() {
        let is_player = player_query.get(*victim).is_ok();

        if let Ok(mut health) = health_query.get_mut(*victim) {
            debug!("Health before attack: {}", health.current);

            health.current -= 1;

            debug!("Health after: {}", health.current);

            if health.current < 1 && !is_player {
                commands.entity(*victim).despawn();
            }
        }
    }
}
