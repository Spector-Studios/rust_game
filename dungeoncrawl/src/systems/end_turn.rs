use crate::resources::EnemyQueue;
use crate::{TurnState, prelude::*};
use bevy_state::{prelude::State, state::NextState};
use std::collections::VecDeque;

pub fn end_turn_system(
    mut commands: Commands,
    turn_state: Res<State<TurnState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    enemy_queue: Res<EnemyQueue>,
    enemy_query: Query<Entity, With<ChasePlayer>>,
    player_query: Query<(&Health, &TilePoint), With<Player>>,
    amulet_query: Query<&TilePoint, With<AmuletOfYala>>,
) {
    info!("turn change start");
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => {
            commands.insert_resource(EnemyQueue(enemy_query.iter().collect::<VecDeque<Entity>>()));
            TurnState::MonsterTurn
        }

        TurnState::MonsterTurn => {
            if enemy_queue.0.is_empty() {
                TurnState::AwaitingInput
            } else {
                return;
            }
        }

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
    info!("turn change end");
}
