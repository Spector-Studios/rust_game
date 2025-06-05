use std::collections::VecDeque;

use crate::PathfindingMap;
use crate::resources::EnemyQueue;
use bracket_pathfinding::prelude::Algorithm2D;
pub mod chasing;
pub mod combat;
pub mod end_turn;
pub mod entity_render;
pub mod hud_render;
pub mod map_render;
pub mod movement;
pub mod player_input;
pub mod random_move;
pub mod update_pathfinding;

use crate::systems::hud_render::hud_render_system;
use bevy_ecs::schedule::ScheduleLabel;

use chasing::chasing_system;
use combat::combat_system;
use end_turn::end_turn_system;
use entity_render::entity_render_system;
use map_render::map_render_system;
use movement::movement_system;
use player_input::player_input_system;
use random_move::random_move_system;
use update_pathfinding::update_pathfinding;

use crate::prelude::*;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct InputSchedule;
pub fn build_input_schedule() -> Schedule {
    let mut schedule = Schedule::new(InputSchedule);
    schedule.add_systems((player_input_system,));

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct PlayerSchedule;
pub fn build_player_schedule() -> Schedule {
    let mut schedule = Schedule::new(PlayerSchedule);
    schedule.add_systems((
        (combat_system, movement_system).before(end_turn_system),
        end_turn_system,
    ));

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct MonsterSchedule;
pub fn build_monster_schedule() -> Schedule {
    let mut schedule = Schedule::new(MonsterSchedule);
    schedule.add_systems((
        (
            update_pathfinding,
            random_move_system,
            chasing_system,
            combat_system,
            movement_system,
        )
            .before(end_turn_system),
        end_turn_system,
    ));

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct RenderSchedule;
pub fn build_render_schedule() -> Schedule {
    let mut schedule = Schedule::new(RenderSchedule);
    schedule.add_systems((map_render_system, entity_render_system, hud_render_system).chain());
    schedule
}

pub fn setup_system(mut commands: Commands) {
    info!("setup start");
    commands.insert_resource(EnemyQueue(VecDeque::new()));

    let sprite_sheet = SpriteSheet {
        // TODO Use Bevy Asset Server
        sprites: Texture2D::from_image(
            &Image::from_file_with_format(
                include_bytes!("../assets/sprites.png"),
                Some(ImageFormat::Png),
            )
            .unwrap(),
        ),
    };

    commands.insert_resource(sprite_sheet);

    commands.insert_resource(Controller::new());
    commands.insert_resource(ButtonState::new());

    let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
    let map_builder = MapBuilder::new(&mut rng);

    let player_idx = map_builder
        .map
        .point2d_to_index(map_builder.player_start.into());

    spawn_player(&mut commands, map_builder.player_start);
    spawn_amulet(&mut commands, map_builder.amulet_start);

    map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|r| r.centre())
        .for_each(|pos| spawn_enemy(&mut commands, &mut rng, pos));

    commands.insert_resource(Viewport::new(map_builder.player_start));
    commands.insert_resource(PathfindingMap::new(&[player_idx], &map_builder.map));
    commands.insert_resource(map_builder.map);

    info!("setup end");
}

pub fn controller_update(mut controller: ResMut<Controller>, mut commands: Commands) {
    controller.update();
    controller.draw();
    commands.insert_resource(controller.button_state);
}
