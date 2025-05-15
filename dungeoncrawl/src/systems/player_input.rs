use crate::{
    TurnState,
    events::{WantsToAttack, WantsToMove},
    prelude::*,
};

pub fn player_input_system(
    //mut frame_time: ResMut<FrameTime>,
    button_state: Res<ButtonState>,
    //map: Res<Map>,
    //mut camera: ResMut<Camera>,
    mut turn_state: ResMut<TurnState>,
    mut player_pos_query: Query<(Entity, &TilePoint, &mut Timer), With<Player>>,
    enemy_pos_query: Query<(Entity, &TilePoint), With<Enemy>>,
    mut move_writer: EventWriter<WantsToMove>,
    mut attack_writer: EventWriter<WantsToAttack>,
) {
    let (player_entity, pos, mut timer) = player_pos_query
        .single_mut()
        .expect("More than one or no players");

    if timer.time < 0.2 {
        timer.time += get_frame_time();
    } else if *button_state != ButtonState::new() && !button_state.back {
        timer.time = 0.0;

        let delta = TilePoint::new(button_state.dpad_x, -(button_state.dpad_y));

        let destination = *pos + delta;

        let mut hit_something = false;

        if destination != TilePoint::zero() {
            enemy_pos_query
                .iter()
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(enemy_entity, _)| {
                    hit_something = true;

                    attack_writer.write(WantsToAttack {
                        attacker: player_entity,
                        victim: enemy_entity,
                    });
                });
        }

        if !hit_something {
            move_writer.write(WantsToMove {
                entity: player_entity,
                destination,
                is_player: true,
            });
        }
        *turn_state = TurnState::PlayerTurn;
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
