use crate::{prelude::*, TurnState};

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
