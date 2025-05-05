mod entity_render;
mod map_render;
mod player_input;

use entity_render::entity_render_system;
use map_render::map_render_system;
use player_input::player_input_system;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        player_input_system,
        map_render_system.after(player_input_system),
        entity_render_system.after(map_render_system),
    ));
    schedule
}
