mod components;
mod events;
mod map;
mod map_builder;
mod player;
mod prelude;
mod resources;
mod spawner;
mod systems;
mod texture_store;
mod viewport;

use std::collections::VecDeque;

use crate::miniquad::conf::Platform;
use crate::miniquad::conf::WebGLVersion;
use crate::prelude::*;
use bracket_pathfinding::prelude::Algorithm2D;
use events::WantsToAttack;
use events::WantsToMove;
use input_lib::Controller;
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

#[derive(Resource, Debug)]
enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn { queue: VecDeque<Entity> },
}

struct Game {
    ecs: World,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    render_systems: Schedule,
    //events: Events<WantsToMove>,
    controller: Controller,
}

impl Game {
    fn new(
        sprite_sheet: SpriteSheet,
        //player_texture: Texture2D,
        //wall_texture: Texture2D,
        //floor_texture: Texture2D,
        //enemy_texture: Texture2D,
    ) -> Self {
        let mut ecs = World::default();

        let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        let map_builder = MapBuilder::new(&mut rng);

        ecs.insert_resource(Viewport::new(map_builder.player_start));
        //ecs.insert_resource(TextureStore::new());
        ecs.insert_resource(sprite_sheet);
        ecs.insert_resource(TurnState::AwaitingInput);
        //ecs.insert_resource(FrameTime(0.0));
        ecs.insert_resource(Events::<WantsToMove>::default());
        ecs.insert_resource(Events::<WantsToAttack>::default());

        let player_idx = map_builder
            .map
            .point2d_to_index(map_builder.player_start.into());
        ecs.insert_resource(PathfindingMap::new(&[player_idx], &map_builder.map));
        ecs.insert_resource(map_builder.map);

        spawn_player(&mut ecs, map_builder.player_start);
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
            //events,
            controller: Controller::new(),
        }
    }
    fn tick(&mut self) {
        self.controller.update(); // TODO Move to ecs
        self.ecs.insert_resource(self.controller.button_state);
        //self.ecs.insert_resource(FrameTime(self.ecs.get_resource::<FrameTime>().unwrap().0 + get_frame_time()));
        match *self.ecs.get_resource::<TurnState>().unwrap() {
            TurnState::AwaitingInput => self.input_systems.run(&mut self.ecs),
            TurnState::PlayerTurn => self.player_systems.run(&mut self.ecs),
            TurnState::MonsterTurn { queue: _ } => self.monster_systems.run(&mut self.ecs),
        }
        self.render_systems.run(&mut self.ecs);

        self.ecs.resource_mut::<Events<WantsToMove>>().update();
        self.ecs.resource_mut::<Events<WantsToAttack>>().update();
        //draw_circle(200.0, 700.0, 90.0, VIOLET);
        //draw_texture(&self.resources.get::<TextureStore>().unwrap().map_render.texture, VIEWPORT_X, VIEWPORT_Y, WHITE);
        self.controller.draw(); // TODO Move to ecs
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // let player_texture = load_texture("resources/Player.png")
    //     .await
    //     .expect("Player texture");
    // let floor_texture = load_texture("resources/Floor.png")
    //     .await
    //     .expect("Floor texture.");
    // let wall_texture = load_texture("resources/Wall.png")
    //     .await
    //     .expect("Wall texture.");
    // let goblin_texture = load_texture("resources/Goblin.png")
    //     .await
    //     .expect("Goblin Texture");

    let sprites = load_texture("resources/sprites.png")
        .await
        .expect("Sprite sheet");
    build_textures_atlas();

    let sprit_sheet = SpriteSheet { sprites };

    let mut game = Game::new(
        sprit_sheet,
        //player_texture,
        //wall_texture,
        //floor_texture,
        //goblin_texture,
    );

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
        platform: Platform {
            webgl_version: WebGLVersion::WebGL2,
            ..Default::default()
        },

        ..Default::default()
    }
}
