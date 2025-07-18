use crate::{prelude::*, TurnState};

/// Advances the game turn state based on the current turn and entity queues.
///
/// Transitions from player to monster turn when the player's action queue is empty, initializing the monster queue with all enemy entities. Transitions from monster turn to awaiting input when the monster queue is empty. Otherwise, the turn state remains unchanged.
///
/// # Examples
///
/// ```
/// // In a Bevy system schedule:
/// end_turn_system(turn_state, enemy_entities);
/// // The turn state will update if the relevant queue is empty.
/// ```
pub fn end_turn_system(
    mut turn_state: ResMut<TurnState>,
    //player_entity: Query<Entity, With<Player>>,
    enemy_entities: Query<Entity, With<Enemy>>,
) {
    let new_state = match &*turn_state {
        TurnState::AwaitingInput => return,

        TurnState::PlayerTurn { queue } if queue.is_empty() => TurnState::MonsterTurn {
            queue: enemy_entities.iter().collect(),
        },
        TurnState::PlayerTurn { queue: _ } => return,

        TurnState::MonsterTurn { queue } if queue.is_empty() => TurnState::AwaitingInput,
        TurnState::MonsterTurn { queue: _ } => return,
    };

    *turn_state = new_state;
}
