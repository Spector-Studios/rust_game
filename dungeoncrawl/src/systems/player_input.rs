use crate::prelude::*;

#[system]
#[write_component(TilePoint)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] input: &ButtonState,
    #[resource] camera: &mut Camera,
) {
    if input == &ButtonState::default() {
        #[cfg(debug_assertions)]
        draw_rectangle(screen_width() - 50.0, 0.0, 50.0, 50.0, YELLOW);

        return;
    }

    //macroquad has "inverted" y axis
    let delta = TilePoint::new(input.dpad_x, -input.dpad_y);
    if delta != TilePoint::zero() {
        let mut players = <&mut TilePoint>::query().filter(component::<Player>());

        players.iter_mut(ecs).for_each(|pos| {
            let destination = *pos + delta;
            if map.can_enter_tile(destination) {
                *pos = destination;
                camera.on_player_move(destination);
            }
        });
    }
}
