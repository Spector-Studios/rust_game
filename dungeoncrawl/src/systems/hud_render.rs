use crate::camera::Camera;
use crate::prelude::*;

pub fn hud_render_system(camera: Res<Camera>, player_health_query: Query<&Health, With<Player>>) {
    let player_health = player_health_query
        .single()
        .expect("No player health component.");

    draw_rectangle(
        camera.get_hud_screen_x(0),
        camera.get_hud_screen_y(0),
        VIEWPORT_WIDTH,
        30.0,
        MAROON,
    );
    draw_rectangle(
        camera.get_hud_screen_x(0),
        camera.get_hud_screen_y(0),
        VIEWPORT_WIDTH * ((player_health.current) as f32 / player_health.max as f32),
        30.0,
        RED,
    );
}
