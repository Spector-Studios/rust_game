use bevy_state::state::NextState;

use crate::{
    TurnState,
    events::{WantsToAttack, WantsToMove},
    prelude::*,
    resources::PathfindingMap,
};

#[allow(clippy::too_many_arguments)]
pub fn player_input_system(
    button_state: Res<ButtonState>,
    mut pathfinding_map: ResMut<PathfindingMap>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut player_query: Query<(Entity, &TilePoint, &mut Health, &mut Timer), With<Player>>,
    enemy_pos_query: Query<(Entity, &TilePoint), With<Enemy>>,
    item_pos_query: Query<(Entity, &TilePoint), With<Item>>,
    mut move_writer: EventWriter<WantsToMove>,
    mut attack_writer: EventWriter<WantsToAttack>,
    mut commands: Commands,
) {
    let (player_entity, player_pos, mut health, mut timer) = player_query
        .single_mut()
        .expect("More than one or no players");

    if timer.time < 0.2 {
        timer.time += get_frame_time();
    } else if button_state.buttons.contains(Buttons::X) {
        item_pos_query
            .iter()
            .filter(|(_, pos)| *pos == player_pos)
            .for_each(|(entity, _)| {
                commands.entity(entity).insert(Carried(player_entity));
                commands.entity(entity).remove::<TilePoint>();
            });
    } else if *button_state != ButtonState::new() && !button_state.buttons.contains(Buttons::B) {
        timer.time = 0.0;
        let mut did_something = false;
        let mut hit_something = false;

        let delta = TilePoint::new(button_state.dpad_x, -(button_state.dpad_y));
        let destination = *player_pos + delta;

        if delta != TilePoint::zero() {
            did_something = true;

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
            pathfinding_map.is_stale = true;
        }

        if !did_something {
            health.current = i32::min(health.max, health.current + 1);
        }

        next_turn_state.set(TurnState::PlayerTurn);
    }

    #[cfg(debug_assertions)]
    draw_text(
        format!("{}, {}", player_pos.x, player_pos.y).as_str(),
        20.0,
        20.0,
        50.0,
        WHITE,
    );
}
