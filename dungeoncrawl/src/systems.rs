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
/// Builds the input schedule containing the player input processing system.
///
/// Returns a `Schedule` labeled with `InputSchedule` that executes the `player_input_system` to handle player input each frame.
///
/// # Examples
///
/// ```
/// let input_schedule = build_input_schedule();
/// // Add the schedule to your Bevy app or ECS dispatcher.
/// ```
pub fn build_input_schedule() -> Schedule {
    let mut schedule = Schedule::new(InputSchedule);
    schedule.add_systems(player_input_system);

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct PlayerSchedule;
/// Builds the player schedule containing systems to finalize the player's turn.
///
/// Returns a `Schedule` labeled with `PlayerSchedule` that executes the `end_turn_system`.
///
/// # Examples
///
/// ```
/// let schedule = build_player_schedule();
/// // Use the schedule in the ECS app
/// ```
pub fn build_player_schedule() -> Schedule {
    let mut schedule = Schedule::new(PlayerSchedule);
    schedule.add_systems((end_turn_system).chain());

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct MonsterSchedule;
/// Builds the schedule for monster AI behavior and turn progression.
///
/// The schedule includes systems for random movement, chasing logic, and ending the monster's turn, executed sequentially.
///
/// # Returns
/// A `Schedule` configured for monster actions and turn completion.
///
/// # Examples
///
/// ```
/// let monster_schedule = build_monster_schedule();
/// ```
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
/// Creates a new animation schedule with no systems.
///
/// Returns a `Schedule` labeled with `AnimationSchedule`, intended for organizing animation-related systems.
///
/// # Examples
///
/// ```
/// let animation_schedule = build_animation_schedule();
/// assert_eq!(animation_schedule.label(), AnimationSchedule);
/// ```
pub fn build_animation_schedule() -> Schedule {
    let schedule = Schedule::new(AnimationSchedule);

    schedule
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct RenderSchedule;
/// Builds the render schedule for drawing the game state.
///
/// The schedule executes map, entity, and HUD rendering systems in sequence to update the visual output.
///
/// # Returns
/// A `Schedule` configured to run the rendering systems in order.
///
/// # Examples
///
/// ```
/// let render_schedule = build_render_schedule();
/// // Use the schedule to run rendering systems in the ECS.
/// ```
pub fn build_render_schedule() -> Schedule {
    let mut schedule = Schedule::new(RenderSchedule);
    schedule.add_systems((map_render_system, entity_render_system, hud_render_system).chain());
    schedule
}
