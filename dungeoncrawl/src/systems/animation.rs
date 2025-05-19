use std::time::Duration;

use crate::prelude::*;

use crate::TurnState;

pub fn animation_tick_system(
    turn_state: Res<TurnState>,
    mut move_animation_query: Query<&mut Animation>,
) {
    let queue = turn_state.get_queue();
    if let Some(entity) = queue.front() {
        if let Ok(mut move_animation) = move_animation_query.get_mut(*entity) {
            move_animation
                .timer
                .tick(Duration::from_secs_f32(get_frame_time()));
        }
    }
}

pub fn animation_end_system(
    mut commands: Commands,
    mut turn_state: ResMut<TurnState>,
    move_animation_query: Query<&Animation>,
) {
    let queue = turn_state.get_mut_queue();
    let Some(entity) = queue.front().cloned() else {
        return;
    };
    if let Ok(move_animation) = move_animation_query.get(entity) {
        if move_animation.timer.finished() {
            queue.pop_front();
            commands.get_entity(entity).unwrap().remove::<Animation>();
        }
    }
}
