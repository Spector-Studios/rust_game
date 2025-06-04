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

use std::collections::VecDeque;
use std::panic;

use crate::prelude::*;
use crate::viewport::Viewport;
use bracket_pathfinding::prelude::Algorithm2D;
use events::WantsToAttack;
use events::WantsToMove;
use input_lib::Controller;
use macroquad::miniquad::conf::Platform;
use macroquad::miniquad::conf::WebGLVersion;
use resources::PathfindingMap;

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

#[derive(Resource, Debug, Clone, PartialEq)]
enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn { queue: VecDeque<Entity> },
    GameOver,
    Victory,
}

struct Game {
    ecs: World,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    render_systems: Schedule,
    controller: Controller,
}

impl Game {
    fn new(sprite_sheet: SpriteSheet) -> Self {
        let mut ecs = World::default();

        let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        let map_builder = MapBuilder::new(&mut rng);

        ecs.insert_resource(Viewport::new(map_builder.player_start));
        ecs.insert_resource(sprite_sheet);
        ecs.insert_resource(TurnState::AwaitingInput);
        ecs.insert_resource(Events::<WantsToMove>::default());
        ecs.insert_resource(Events::<WantsToAttack>::default());

        let player_idx = map_builder
            .map
            .point2d_to_index(map_builder.player_start.into());
        ecs.insert_resource(PathfindingMap::new(&[player_idx], &map_builder.map));
        ecs.insert_resource(map_builder.map);

        spawn_player(&mut ecs, map_builder.player_start);
        spawn_amulet(&mut ecs, map_builder.amulet_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.centre())
            .for_each(|pos| spawn_enemy(&mut ecs, &mut rng, pos));

        Self {
            ecs,
            input_systems: build_input_schedule(),
            player_systems: build_player_schedule(),
            monster_systems: build_monster_schedule(),
            render_systems: build_render_schedule(),
            controller: Controller::new(),
        }
    }

    fn game_over(&mut self, buttons: ButtonState) {
        let msg = "Game Over";
        let text_centre = get_text_center(msg, None, 200, 1.0, 0.0);
        draw_text(
            msg,
            Viewport::viewport_centre().x - text_centre.x,
            Viewport::viewport_centre().y - text_centre.y,
            200.0,
            RED,
        );

        if buttons.action {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, buttons: ButtonState) {
        let msg = "VICTORY";
        let text_centre = get_text_center(msg, None, 200, 1.0, 0.0);
        draw_text(
            msg,
            Viewport::viewport_centre().x - text_centre.x,
            Viewport::viewport_centre().y - text_centre.y,
            200.0,
            GREEN,
        );

        if buttons.action {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs.clear_entities();
        let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        let map_builder = MapBuilder::new(&mut rng);

        self.ecs
            .insert_resource(Viewport::new(map_builder.player_start));
        self.ecs.insert_resource(TurnState::AwaitingInput);
        self.ecs.insert_resource(Events::<WantsToMove>::default());
        self.ecs.insert_resource(Events::<WantsToAttack>::default());

        let player_idx = map_builder
            .map
            .point2d_to_index(map_builder.player_start.into());
        self.ecs
            .insert_resource(PathfindingMap::new(&[player_idx], &map_builder.map));
        self.ecs.insert_resource(map_builder.map);

        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet(&mut self.ecs, map_builder.amulet_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.centre())
            .for_each(|pos| spawn_enemy(&mut self.ecs, &mut rng, pos));
    }

    fn tick(&mut self) {
        self.controller.update(); // TODO Move to ecs
        self.ecs.insert_resource(self.controller.button_state);

        let mut render_game = true;
        match self.ecs.get_resource::<TurnState>().unwrap() {
            TurnState::AwaitingInput => self.input_systems.run(&mut self.ecs),
            TurnState::PlayerTurn => self.player_systems.run(&mut self.ecs),
            TurnState::MonsterTurn { .. } => self.monster_systems.run(&mut self.ecs),
            TurnState::GameOver => {
                let buttons = *self.ecs.get_resource::<ButtonState>().unwrap();
                self.game_over(buttons);
                render_game = false;
            }
            TurnState::Victory => {
                let buttons = *self.ecs.get_resource::<ButtonState>().unwrap();
                self.victory(buttons);
                render_game = false;
            }
        }

        if render_game {
            self.render_systems.run(&mut self.ecs);
        }

        self.ecs.resource_mut::<Events<WantsToMove>>().update();
        self.ecs.resource_mut::<Events<WantsToAttack>>().update();

        self.controller.draw(); // TODO Move to ecs
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    panic::set_hook(Box::new(|info| error!("{}", info)));
    set_pc_assets_folder("assets");

    let sprites = load_texture("sprites.png")
        .await
        .expect("Sprite sheet");
    build_textures_atlas();

    let sprit_sheet = SpriteSheet { sprites };

    let mut game = Game::new(sprit_sheet);

    let render_target = render_target(380, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let materialtest = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![UniformDesc::new("iResolution", UniformType::Float2)],
            ..Default::default()
        },
    );

    if materialtest.is_err() {
        debug!("{}", materialtest.unwrap_err());
    }
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![UniformDesc::new("iResolution", UniformType::Float2)],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(BLACK);

        /* material.set_uniform("iResolution", (screen_width(), screen_height()));
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material(); */

        game.tick();

        draw_rectangle_lines(
            (screen_width() - VIEWPORT_WIDTH) / 2.0,
            ((screen_height() - VIEWPORT_HEIGHT) / 2.0) * 0.7,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            10.0,
            WHITE,
        );
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Test".to_owned(),
        // high_dpi: true,
        platform: Platform {
            webgl_version: WebGLVersion::WebGL2,
            ..Default::default()
        },

        ..Default::default()
    }
}
