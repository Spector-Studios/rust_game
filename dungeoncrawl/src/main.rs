mod components;
mod events;
mod map;
mod map_builder;
mod prelude;
mod resources;
mod spawner;
mod systems;
mod texture_store;
mod viewport;

use crate::movement::movement_system;
use crate::prelude::*;
use bevy_app::prelude::*;
use bevy_ecs::error::GLOBAL_ERROR_HANDLER;
use bevy_state::app::AppExtStates;
use bevy_state::app::StatesPlugin;
use bevy_state::prelude::in_state;
use bevy_state::state::States;
use events::WantsToAttack;
use events::WantsToMove;
use macroquad::miniquad::conf::Platform;
use macroquad::miniquad::conf::WebGLVersion;
use macroquad::ui;
use prelude::chasing::chasing_system;
use prelude::combat::combat_system;
use prelude::end_turn::end_turn_system;
use prelude::entity_render::entity_render_system;
use prelude::fov::fov;
use prelude::hud_render::hud_render_system;
use prelude::map_render::map_render_system;
use prelude::random_move::random_move_system;
use prelude::update_pathfinding::update_pathfinding;
use resources::PathfindingMap;
use std::f32::consts::PI;
use std::panic;
use systems::player_input::player_input_system;

const FRAGMENT_SHADER: &str = "
#version 100
precision mediump float;

uniform vec2 iResolution;

void main(void) {
    vec2 uv = gl_FragCoord.xy / iResolution.xy;
    gl_FragColor = vec4(uv, 1.0, 1.0);
}
";

const VERTEX_SHADER: &str = "
#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash, Default, States)]
enum TurnState {
    #[default]
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
}

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {s:?}");
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic occurred: {s:?}");
        } else {
            error!("panic occurred");
        }

        error!("{}", panic_info);
    }));

    GLOBAL_ERROR_HANDLER
        .set(|error, context| {
            error!("{}", error);
            error!("{}", context);
            panic!("{}\n{}", error, context);
        })
        .expect("Error");

    let mut app = App::new();
    app.add_plugins(StatesPlugin)
        .add_plugins(MacroquadRunner("Hello"))
        //.init_state::<TurnState>()
        .insert_state(TurnState::AwaitingInput)
        .add_event::<WantsToAttack>()
        .add_event::<WantsToMove>()
        .add_systems(Startup, setup_system)
        .add_systems(PreUpdate, controller_update)
        // TODO Add systems for GameOver and Victory states
        .add_systems(
            Update,
            player_input_system.run_if(in_state(TurnState::AwaitingInput)),
        )
        .add_systems(
            Update,
            (combat_system, movement_system, end_turn_system)
                .chain()
                .run_if(in_state(TurnState::PlayerTurn)),
        )
        .add_systems(
            Update,
            (
                update_pathfinding,
                //random_move_system,
                chasing_system,
                combat_system,
                movement_system,
                end_turn_system,
            )
                .chain()
                .run_if(in_state(TurnState::MonsterTurn)),
        )
        //.add_systems(Update, (combat_system, movement_system, end_turn_system))
        .add_systems(
            Update,
            (map_render_system, entity_render_system, hud_render_system).chain(),
        )
        .add_systems(
            Update,
            restart_system.run_if(in_state(TurnState::GameOver).or(in_state(TurnState::Victory))),
        )
        // TODO Add computed state for this
        .add_systems(
            Update,
            fov.run_if(not(
                in_state(TurnState::GameOver).or(in_state(TurnState::Victory))
            )),
        )
        .run();
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Test".to_owned(),

        #[cfg(not(target_family = "wasm"))]
        high_dpi: true,
        platform: Platform {
            webgl_version: WebGLVersion::WebGL2,
            ..Default::default()
        },

        ..Default::default()
    }
}

pub struct MacroquadRunner(pub &'static str);
impl Plugin for MacroquadRunner {
    fn build(&self, app: &mut App) {
        app.set_runner(macroquad_runner);
    }
}

fn macroquad_runner(mut app: App) -> AppExit {
    app.finish();
    app.cleanup();

    macroquad::Window::from_config(window_conf(), async move {
        set_panic_handler(|msg, backtrace| async move {
            loop {
                clear_background(RED);
                ui::root_ui().label(None, &msg);
                for line in backtrace.split('\n') {
                    ui::root_ui().label(None, line);
                }
                next_frame().await;
            }
        });

        // DEFAULTS
        set_pc_assets_folder("assets");
        set_default_filter_mode(FilterMode::Nearest);

        let sprite_sheet = SpriteSheet::default().await.unwrap();

        // XXX WARNING Finish loading all the textures before this
        build_textures_atlas();
        // XXX WARNING -------------------------------------------

        app.insert_resource(sprite_sheet);

        let mut x: f32 = 0.1;
        loop {
            clear_background(BLUE);
            x = (x + 0.001).fract();
            draw_rectangle_ex(
                400.0,
                400.0,
                300.0,
                300.0,
                DrawRectangleParams {
                    rotation: x * PI * 2.0,
                    color: RED,
                    ..Default::default()
                },
            );
            app.update();
            next_frame().await;
        }
    });
    bevy_app::AppExit::Success
}
