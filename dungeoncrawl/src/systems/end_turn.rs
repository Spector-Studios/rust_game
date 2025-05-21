use crate::{TurnState, prelude::*};

pub fn end_turn_system(
    mut turn_state: ResMut<TurnState>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_health: Query<&Health, With<Player>>,
) {
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn {
            queue: enemy_query.iter().collect(),
        },

        TurnState::MonsterTurn { queue } if queue.is_empty() => TurnState::AwaitingInput,
        TurnState::MonsterTurn { queue: _ } => return,

        _ => current_state,
    };

    if player_health.single().unwrap().current < 1 {
        new_state = TurnState::GameOver;
    }

    *turn_state = new_state;
}
