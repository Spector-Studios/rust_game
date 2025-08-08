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

use crate::events::ActivateItem;
use crate::movement::movement_system;
use crate::prelude::advance_level::advance_level;
use crate::prelude::event_readers::use_item;
use crate::prelude::player_input::player_menu_input_system;
use crate::prelude::template::Templates;
use crate::prelude::*;
use crate::resources::FontResource;
use bevy_app::prelude::*;
use bevy_ecs::error::GLOBAL_ERROR_HANDLER;
use bevy_state::app::AppExtStates;
use bevy_state::app::StatesPlugin;
use bevy_state::prelude::*;
use events::WantsToAttack;
use events::WantsToMove;
use macroquad::ui;
use prelude::chasing::chasing_system;
use prelude::combat::combat_system;
use prelude::end_turn::end_turn_system;
use prelude::entity_render::entity_render_system;
use prelude::fov::fov;
use prelude::hud_render::hud_render_system;
use prelude::map_render::map_render_system;
use prelude::update_pathfinding::update_pathfinding;
use resources::PathfindingMap;
use std::panic;
use systems::player_input::player_move_input_system;

/* const FRAGMENT_SHADER: &str = "
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
"; */

#[cfg(target_os = "android")]
mod __android_glue {
    #[unsafe(no_mangle)]
    pub extern "C" fn quad_main() {
        std::panic::catch_unwind(super::main).unwrap_or_else(|_| {
            // TODO Call into JAVA to exit the app
            std::process::exit(1);
        });
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_glue {
    #[unsafe(no_mangle)]
    pub extern "C" fn main() {
        super::main();
    }
}

#[derive(States, Resource, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum TurnState {
    #[default]
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}

#[derive(SubStates, Resource, Clone, PartialEq, Eq, Hash, Default, Debug)]
#[source(TurnState = TurnState::AwaitingInput)]
enum InMenu {
    #[default]
    Move,
    Menu,
}

fn build_app() -> App {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {s:?}");
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic occurred: {s:?}");
        } else {
            error!("panic occurred");
        }

        error!("{}", panic_info);

        std::process::exit(1);
    }));

    GLOBAL_ERROR_HANDLER
        .set(|error, context| {
            error!("{}", error);
            error!("{}", context);
            panic!("{error}\n{context}");
        })
        .expect("Error");

    let mut app = App::new();
    app.add_plugins(StatesPlugin)
        // .add_plugins(MacroquadRunner("Hello"))
        //.init_state::<TurnState>()
        .insert_state(TurnState::AwaitingInput)
        .add_sub_state::<InMenu>()
        .add_event::<WantsToAttack>()
        .add_event::<WantsToMove>()
        .add_event::<ActivateItem>()
        .add_systems(Startup, setup_system)
        .add_systems(PreUpdate, controller_update)
        .add_systems(Update, use_item)
        .add_systems(
            Update,
            player_move_input_system.run_if(in_state(InMenu::Move)),
        )
        .add_systems(
            Update,
            player_menu_input_system.run_if(in_state(InMenu::Menu)),
        )
        // TODO Use System Configs to better determine order irrespective of state
        // E.g. fov system needs to run after movement system to avoid flickering of map
        .add_systems(
            Update,
            (combat_system, movement_system, fov, end_turn_system)
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
            PostUpdate,
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
        .add_systems(Update, advance_level.run_if(in_state(TurnState::NextLevel)));

    app.finish();
    app.cleanup();
    app
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Test".to_owned(),
        // #[cfg(not(target_family = "wasm"))]
        // high_dpi: true,
        // platform: Platform {
        //     webgl_version: WebGLVersion::WebGL2,
        //     ..Default::default()
        // },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() {
    let mut app = build_app();

    set_panic_handler(|msg, backtrace| async move {
        loop {
            clear_background(BLACK);
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

    async_bevy_setup(&mut app).await;

    loop {
        macroquad_loop(&mut app);
        next_frame().await;
    }
}

fn macroquad_loop(app: &mut App) {
    clear_background(BLACK);
    app.update();

    draw_rectangle_lines(
        Viewport::x_offset(),
        Viewport::y_offset(),
        VIEWPORT_WIDTH,
        VIEWPORT_HEIGHT,
        10.0,
        WHITE,
    );
}

async fn async_bevy_setup(app: &mut App) {
    let sprite_sheet = SpriteSheet::new().await;
    let font = load_ttf_font("font.ttf").await.unwrap();

    // XXX WARNING Finish loading all the textures before this
    build_textures_atlas();
    // XXX WARNING -------------------------------------------

    app.insert_resource(sprite_sheet);
    app.insert_resource(FontResource(font));

    let template = Templates::load().await;
    app.insert_resource(template);
}
