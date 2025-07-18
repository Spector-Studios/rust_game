use std::time::Duration;

use crate::prelude::*;

use crate::TurnState;

/// Advances the animation timer for the entity at the front of the turn queue.
///
/// If the entity at the front of the turn queue has an `Animation` component, its timer is advanced by the current frame's duration.
///
/// # Examples
///
/// ```
/// // System usage within a Bevy schedule:
/// app.add_system(animation_tick_system);
/// ```
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

/// Ends the animation for the entity at the front of the turn queue if its animation timer has finished.
///
/// Removes the `Animation` component from the entity and advances the turn queue when the animation completes. Does nothing if the queue is empty or the animation is not finished.
///
/// # Examples
///
/// ```
/// // System usage in Bevy schedule:
/// app.add_system(animation_end_system);
/// ```
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
