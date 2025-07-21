use crate::{TurnState, prelude::*};
use bevy_state::{prelude::State, state::NextState};
use bracket_pathfinding::prelude::Algorithm2D;

pub fn end_turn_system(
    turn_state: Res<State<TurnState>>,
    map: Res<Map>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    player_query: Query<(&Health, &TilePoint), With<Player>>,
    amulet_query: Query<&TilePoint, With<AmuletOfYala>>,
) {
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::GameOver | TurnState::Victory => return,
        TurnState::NextLevel => current_state,
    };

    let (player_health, player_pos) = player_query.single().unwrap();
    if player_health.current < 1 {
        new_state = TurnState::GameOver;
    }

    let amulet_default = TilePoint::new(-1, -1);
    let amulet_pos = amulet_query.single().unwrap_or(&amulet_default);
    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }

    let idx = map.point2d_to_index((*player_pos).into());
    if map.tiles[idx] == TileType::Stair {
        new_state = TurnState::NextLevel;
    }

    next_turn_state.set(new_state);
}
