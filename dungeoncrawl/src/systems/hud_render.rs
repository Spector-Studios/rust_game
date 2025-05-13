use crate::prelude::*;

pub fn hud_render_system(
    button_state: Res<ButtonState>,
    viewport: Res<Viewport>,
    player_health_query: Query<&Health, With<Player>>,
    enemy_query: Query<(&EnemyName, &TilePoint, &Health), With<Enemy>>,
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
            .for_each(|(name, pos, health)| {
                draw_text(
                    format!("{}\n{}", name.0, health.current).as_str(),
                    viewport.get_screen_x(pos.x),
                    viewport.get_screen_y(pos.y),
                    30.0,
                    BLACK,
                );
            });
    }
}
