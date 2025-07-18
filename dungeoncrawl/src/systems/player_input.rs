use std::collections::VecDeque;

use crate::{prelude::*, TurnState};

/// Processes player input, updates the player's state, and advances the game turn.
///
/// Handles movement input by inserting a movement animation on the player entity if a directional input is detected. If no movement occurs, the player's health regenerates by one point up to the maximum. Advances the turn state to the player's turn with the player entity in the queue. Input is processed only if the input timer exceeds 0.2 seconds and the back button is not pressed. Panics if there is not exactly one player entity.
///
/// # Panics
///
/// Panics if the player query does not return exactly one player entity.
///
/// # Examples
///
/// ```
/// // This system would be added to the Bevy schedule:
/// app.add_system(player_input_system);
/// ```
pub fn player_input_system(
    //mut frame_time: ResMut<FrameTime>,
    button_state: Res<ButtonState>,
    mut commands: Commands,
    //map: Res<Map>,
    //mut camera: ResMut<Camera>,
    mut turn_state: ResMut<TurnState>,
    mut player_query: Query<(Entity, &TilePoint, &mut Health, &mut InputTimer), With<Player>>,
    //enemy_pos_query: Query<(Entity, &TilePoint), With<Enemy>>,
    //mut move_writer: EventWriter<WantsToMove>,
    //mut attack_writer: EventWriter<WantsToAttack>,
) {
    let (player_entity, pos, mut health, mut timer) = player_query
        .single_mut()
        .expect("More than one or no players");

    if timer.time < 0.2 {
        timer.time += get_frame_time();
    } else if *button_state != ButtonState::new() && !button_state.back {
        timer.time = 0.0;
        let mut did_something = false;

        let delta = TilePoint::new(button_state.dpad_x, -(button_state.dpad_y));

        if delta != TilePoint::zero() {
            did_something = true;
            commands
                .entity(player_entity)
                .insert(Animation::new_movement(*pos, *pos + delta));
        }

        if !did_something {
            health.current = i32::min(health.max, health.current + 1);
        }
        *turn_state = TurnState::PlayerTurn {
            queue: VecDeque::from([player_entity]),
        };
    }

    #[cfg(debug_assertions)]
    draw_text(
        format!("{}, {}", pos.x, pos.y).as_str(),
        20.0,
        20.0,
        50.0,
        WHITE,
    );
}
