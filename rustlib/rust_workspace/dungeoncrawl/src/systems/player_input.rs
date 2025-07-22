use bevy_state::state::NextState;

use crate::{
    InMenu, TurnState,
    events::{ActivateItem, WantsToAttack, WantsToMove},
    prelude::*,
    resources::PathfindingMap,
};

#[allow(clippy::too_many_arguments)]
pub fn player_move_input_system(
    button_state: Res<ButtonState>,
    mut pathfinding_map: ResMut<PathfindingMap>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_menu_state: ResMut<NextState<InMenu>>,
    mut player_query: Query<(Entity, &TilePoint, &mut Timer), With<Player>>,
    enemy_pos_query: Query<(Entity, &TilePoint), With<Enemy>>,
    item_pos_query: Query<(Entity, &TilePoint), With<Item>>,
    mut move_writer: EventWriter<WantsToMove>,
    mut attack_writer: EventWriter<WantsToAttack>,
    mut commands: Commands,
) {
    let (player_entity, player_pos, mut timer) = player_query
        .single_mut()
        .expect("More than one or no players");

    if timer.time < 0.2 {
        timer.time += get_frame_time();
    } else if button_state.buttons.contains(Buttons::Y) {
        panic!("test");
    } else if button_state.buttons.contains(Buttons::Select) {
        timer.time = 0.0;
        commands.entity(player_entity).insert(SelectedItemIndex(0));
        next_menu_state.set(InMenu::Menu);
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
        let mut hit_something = false;

        let delta = TilePoint::new(button_state.dpad_x, -(button_state.dpad_y));
        let destination = *player_pos + delta;

        if delta != TilePoint::zero() {
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

pub fn player_menu_input_system(
    button_state: Res<ButtonState>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut next_menu_state: ResMut<NextState<InMenu>>,
    mut player_query: Query<(Entity, &mut SelectedItemIndex, &mut Timer), With<Player>>,
    carried_items: Query<(Entity, &Carried), With<Item>>,
    mut item_event: EventWriter<ActivateItem>,
    mut commands: Commands,
) {
    let (player_entity, mut selected_index, mut timer) = player_query
        .single_mut()
        .expect("More than one or no players");

    let mut item_count: usize = 0;

    let mut selected_item = None;
    for (entity, carried) in carried_items.iter() {
        if carried.0 != player_entity {
            continue;
        }

        if item_count == **selected_index {
            selected_item = Some(entity);
        }

        item_count += 1;
    }

    if timer.time < 0.4 {
        timer.time += get_frame_time();
    } else if button_state.buttons.contains(Buttons::Select) {
        timer.time = 0.0;
        commands.entity(player_entity).remove::<SelectedItemIndex>();
        next_menu_state.set(InMenu::Move);
    } else if button_state.buttons.contains(Buttons::A) {
        if let Some(item) = selected_item {
            item_event.write(ActivateItem {
                used_by: player_entity,
                item,
            });

            #[cfg(debug_assertions)]
            info!("Item used");

            commands.entity(player_entity).remove::<SelectedItemIndex>();
            next_turn_state.set(TurnState::PlayerTurn);
        }
    } else if button_state.dpad_y != 0 {
        timer.time = 0.0;
        **selected_index = selected_index
            .saturating_add_signed((-button_state.dpad_y) as isize)
            .min(item_count.saturating_sub(1));

        #[cfg(debug_assertions)]
        info!("Item changed");
    }
}
