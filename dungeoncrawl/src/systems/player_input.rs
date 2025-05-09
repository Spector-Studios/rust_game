use crate::{TurnState, camera::Camera, events::WantsToMove, prelude::*};

pub fn player_input_system(
    //mut frame_time: ResMut<FrameTime>,
    mut button_state: ResMut<ButtonState>,
    map: Res<Map>,
    mut camera: ResMut<Camera>,
    mut turn_state: ResMut<TurnState>,
    mut player_pos_query: Query<(Entity, &TilePoint, &mut Timer), With<Player>>,
    mut writer: EventWriter<WantsToMove>,
) {
    let (entity, pos, mut timer) = player_pos_query
        .single_mut()
        .expect("More than one or no players");

    if timer.time < 0.2 {
        timer.time += get_frame_time();
    } else if *button_state != ButtonState::new() {
        timer.time = 0.0;

        let delta = TilePoint::new(
            button_state.dpad_x.clamp(-1, 1),
            -(button_state.dpad_y.clamp(-1, 1)),
        );

        // //if delta != TilePoint::zero() {
        let destination = *pos + delta;

        // if map.can_enter_tile(destination) {
        //     *pos = destination;
        //     camera.on_player_move(destination);
        //     *turn_state = TurnState::PlayerTurn;
        // }
        // //}

        writer.write(WantsToMove {
            entity,
            destination,
        });
        *turn_state = TurnState::PlayerTurn;
    }

    button_state.reset();

    draw_text(
        format!("{}, {}", pos.x, pos.y).as_str(),
        20.0,
        20.0,
        50.0,
        BLACK,
    );
}
