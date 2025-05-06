mod collisions;
mod entity_render;
mod map_render;
mod player_input;

use collisions::player_collision_system;
use entity_render::entity_render_system;
use map_render::map_render_system;
use player_input::player_input_system;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        player_input_system,
        player_collision_system.after(player_input_system),
        map_render_system.after(player_input_system),
        entity_render_system.after(player_collision_system),
    ));
    schedule
}
