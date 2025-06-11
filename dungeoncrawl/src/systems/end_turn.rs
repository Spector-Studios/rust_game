use crate::{TurnState, prelude::*};
use bevy_state::{prelude::State, state::NextState};

pub fn end_turn_system(
    turn_state: Res<State<TurnState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    player_query: Query<(&Health, &TilePoint), With<Player>>,
    amulet_query: Query<&TilePoint, With<AmuletOfYala>>,
) {
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => return,
    };

    let (player_health, player_pos) = player_query.single().unwrap();
    let amulet_pos = amulet_query.single().unwrap();
    if player_health.current < 1 {
        new_state = TurnState::GameOver;
    }

    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }

    next_turn_state.set(new_state);
}
