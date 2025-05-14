use crate::prelude::*;

pub fn hud_render_system(
    button_state: Res<ButtonState>,
    viewport: Res<Viewport>,
    player_health_query: Query<&Health, With<Player>>,
    enemy_query: Query<(&EntityName, &TilePoint, Option<&Health>), Without<Player>>,
) {
    let player_health = player_health_query
        .single()
        .expect("No player health component.");

    draw_rectangle(
        viewport.get_hud_screen_x(0),
        viewport.get_hud_screen_y(0),
        VIEWPORT_WIDTH,
        30.0,
        MAROON,
    );
    draw_rectangle(
        viewport.get_hud_screen_x(0),
        viewport.get_hud_screen_y(0),
        VIEWPORT_WIDTH * ((player_health.current) as f32 / player_health.max as f32),
        30.0,
        RED,
    );

    if button_state.back {
        enemy_query
            .iter()
            .filter(|(_, pos, _)| viewport.view_area.contains(**pos))
            .for_each(|(EntityName(name), pos, option_health)| {
                draw_text(
                    name.as_str(),
                    viewport.get_screen_x(pos.x),
                    viewport.get_screen_y(pos.y) + 50.0,
                    30.0,
                    WHITE,
                );
                if let Some(health) = option_health {
                    draw_text(
                        health.current.to_string().as_str(),
                        viewport.get_screen_x(pos.x),
                        viewport.get_screen_y(pos.y) + 70.0,
                        30.0,
                        WHITE,
                    );
                }
            });
    }
}
