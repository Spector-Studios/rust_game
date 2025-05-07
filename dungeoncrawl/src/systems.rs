mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use collisions::player_collision_system;
use entity_render::entity_render_system;
use map_render::map_render_system;
use player_input::player_input_system;
use random_move::random_move_system;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        player_input_system,
        player_collision_system.after(player_input_system),
        map_render_system.after(player_input_system),
        random_move_system.after(player_collision_system),
        entity_render_system.after(random_move_system),
    ));
    schedule
}
