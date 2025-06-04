use crate::{TurnState, prelude::*};

pub fn end_turn_system(
    mut turn_state: ResMut<TurnState>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<(&Health, &TilePoint), With<Player>>,
    amulet_query: Query<&TilePoint, With<AmuletOfYala>>,
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

    let (player_health, player_pos) = player_query.single().unwrap();
    let amulet_pos = amulet_query.single().unwrap();
    if player_health.current < 1 {
        new_state = TurnState::GameOver;
    }

    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }

    *turn_state = new_state;
}
