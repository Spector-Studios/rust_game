use crate::{prelude::*, TurnState};

pub fn end_turn_system(mut turn_state: ResMut<TurnState>) {
    let new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    *turn_state = new_state;
}
