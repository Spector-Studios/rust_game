mod camera;
mod components;
mod map;
mod map_builder;
mod player;
mod prelude;
mod spawner;
mod systems;
mod texture_store;

use crate::prelude::*;
use input_lib::Controller;

#[derive(Resource, Debug)]
enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

struct Game {
    ecs: World,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
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

        ecs.insert_resource(map_builder.map);
        ecs.insert_resource(Camera::new(map_builder.player_start));
        //ecs.insert_resource(TextureStore::new());
        ecs.insert_resource(sprite_sheet);
        ecs.insert_resource(TurnState::AwaitingInput);
        //ecs.insert_resource(FrameTime(0.0));

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
            TurnState::MonsterTurn => self.monster_systems.run(&mut self.ecs)
        }

        //draw_circle(200.0, 700.0, 90.0, VIOLET);
        //draw_texture(&self.resources.get::<TextureStore>().unwrap().map_render.texture, VIEWPORT_X, VIEWPORT_Y, WHITE);
        self.controller.draw(); // TODO Move to ecs
    }
}

#[macroquad::main("Dungeon Crawl")]
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
    let sprit_sheet = SpriteSheet { sprites };

    let mut game = Game::new(
        sprit_sheet,
        //player_texture,
        //wall_texture,
        //floor_texture,
        //goblin_texture,
    );

    loop {
        clear_background(SKYBLUE);
        game.tick();

        draw_rectangle_lines(VIEWPORT_X, VIEWPORT_Y, VIEWPORT_WIDTH, VIEWPORT_HEIGHT, 10.0, BLACK);
        next_frame().await;
    }
}
