// TODO Do better organisation of these
pub mod chasing;
pub mod combat;
pub mod end_turn;
pub mod entity_render;
pub mod fov;
pub mod hud_render;
pub mod map_render;
pub mod movement;
pub mod player_input;
pub mod random_move;
pub mod update_pathfinding;

use crate::PathfindingMap;
use crate::events::{WantsToAttack, WantsToMove};
use crate::resources::FontResource;
use crate::{TurnState, prelude::*};
use bevy_app::Startup;
use bevy_ecs::system::SystemState;
use bevy_state::commands::CommandsStatesExt;
use bracket_pathfinding::prelude::Algorithm2D;

// TODO Make a better way to restart
pub fn setup_system(world: &mut World, p_commands: &mut SystemState<Commands>) {
    info!("setup start");

    world.clear_entities();
    world
        .get_resource_mut::<Events<WantsToAttack>>()
        .unwrap()
        .clear();
    world
        .get_resource_mut::<Events<WantsToMove>>()
        .unwrap()
        .clear();

    {
        let mut commands = p_commands.get_mut(world);

        commands.insert_resource(Controller::new());
        commands.insert_resource(input_lib::ButtonState::new());

        let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        let map_builder = MapBuilder::new(&mut rng);

        let player_idx = map_builder
            .map
            .point2d_to_index(map_builder.player_start.into());

        spawn_player(&mut commands, map_builder.player_start);
        spawn_amulet(&mut commands, map_builder.amulet_start);

        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut commands, *pos, &mut rng));

        commands.insert_resource(Viewport::new(map_builder.player_start));
        commands.insert_resource(PathfindingMap::new(&[player_idx], &map_builder.map));
        commands.insert_resource(map_builder.theme);
        commands.insert_resource(map_builder.map);

        commands.set_state(TurnState::AwaitingInput);
    }
    p_commands.apply(world);

    info!("setup end");
}

pub fn controller_update(
    mut controller: ResMut<Controller>,
    mut commands: Commands,
    font: Res<FontResource>,
) {
    controller.update();
    controller.draw(Some(&font));
    commands.insert_resource(controller.button_state);
}

pub fn restart_system(button_state: Res<ButtonState>, mut commands: Commands) {
    if button_state.buttons.contains(Buttons::A) {
        commands.run_schedule(Startup);
    }
}
