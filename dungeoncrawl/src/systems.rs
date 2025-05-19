mod animation;
mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod hud_render;
mod map_render;
mod movement;
mod player_input;
mod random_move;

use bevy_ecs::schedule::ScheduleLabel;

use chasing::chasing_system;
use combat::combat_system;
use end_turn::end_turn_system;
use entity_render::entity_render_system;
use hud_render::hud_render_system;
use map_render::map_render_system;
use movement::movement_system;
use player_input::player_input_system;
use random_move::random_move_system;

use crate::prelude::*;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct InputSchedule;
pub fn build_input_schedule() -> Schedule {
    let mut schedule = Schedule::new(InputSchedule);
    schedule.add_systems(player_input_system);

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct PlayerSchedule;
pub fn build_player_schedule() -> Schedule {
    let mut schedule = Schedule::new(PlayerSchedule);
    schedule.add_systems((end_turn_system).chain());

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct MonsterSchedule;
pub fn build_monster_schedule() -> Schedule {
    let mut schedule = Schedule::new(MonsterSchedule);
    schedule.add_systems(
        (
            random_move_system,
            chasing_system,
            //combat_system,
            //movement_system,
            end_turn_system,
        )
            .chain(),
    );

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct AnimationSchedule;
pub fn build_animation_schedule() -> Schedule {
    let schedule = Schedule::new(AnimationSchedule);

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct RenderSchedule;
pub fn build_render_schedule() -> Schedule {
    let mut schedule = Schedule::new(RenderSchedule);
    schedule.add_systems((map_render_system, entity_render_system, hud_render_system).chain());
    schedule
}
