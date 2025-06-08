use crate::{prelude::*, TurnState};
use bevy_state::prelude::State;

pub fn hud_render_system(
    button_state: Res<ButtonState>,
    viewport: Res<Viewport>,
    turn_state: Res<State<TurnState>>,
    player_query: Query<(&Health, &FieldOfView), With<Player>>,
    enemy_query: Query<(&EntityName, &TilePoint, Option<&Health>), Without<Player>>,
) {
    let (player_health, player_fov) = player_query.single().expect("No player health component.");

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

    let msg = match **turn_state {
        TurnState::AwaitingInput => "",
        TurnState::PlayerTurn => "Processing",
        TurnState::MonsterTurn => "Processing",
        TurnState::GameOver => "Game Over",
        TurnState::Victory => "Victory",
    };

    let centre = get_text_center(msg, None, 100, 1.0, 0.0);
    draw_text(
        msg,
        VIEWPORT_WIDTH / 2.0 + Viewport::x_offset() - centre.x,
        VIEWPORT_HEIGHT / 2.0 + Viewport::y_offset() - centre.y,
        100.0,
        WHITE,
    );

    if button_state.back {
        enemy_query
            .iter()
            .filter(|(_, pos, _)| {
                viewport.view_area.contains(**pos)
                    && player_fov.visible_tiles.contains(&(**pos).into())
            })
            .for_each(|(EntityName(name), pos, option_health)| {
                let centre = get_text_center(name.as_str(), None, 30, 1.0, 0.0);

                draw_text(
                    name.as_str(),
                    viewport.get_screen_x(pos.x) + TILE_SIZE / 2.0 - centre.x,
                    viewport.get_screen_y(pos.y) + 50.0,
                    30.0,
                    WHITE,
                );

                if let Some(health) = option_health {
                    let centre =
                        get_text_center(health.current.to_string().as_str(), None, 30, 1.0, 0.0);

                    draw_text(
                        health.current.to_string().as_str(),
                        viewport.get_screen_x(pos.x) + TILE_SIZE / 2.0 - centre.x,
                        viewport.get_screen_y(pos.y) + 70.0,
                        30.0,
                        WHITE,
                    );
                }
            });
    }
}
