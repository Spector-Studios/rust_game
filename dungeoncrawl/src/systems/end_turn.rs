use crate::{TurnState, prelude::*};

pub fn end_turn_system(mut turn_state: ResMut<TurnState>, enemy_query: Query<Entity, With<Enemy>>) {
    let new_state = match &*turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn {
            queue: enemy_query.iter().collect(),
        },

        TurnState::MonsterTurn { queue } if queue.is_empty() => TurnState::AwaitingInput,
        TurnState::MonsterTurn { queue: _ } => return,
    };

    *turn_state = new_state;
}
