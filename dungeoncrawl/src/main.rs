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

macro_rules! texture_from_file {
    ($path:literal) => {
        load_texture($path).await.expect("$path texture")
    };
}

struct Game {
    ecs: World,
    systems: Schedule,
    controller: Controller,
}

impl Game {
    fn new(texture_store: TextureStore, floor_texture: Texture2D, wall_texture: Texture2D) -> Self {
        let mut ecs = World::default();

        let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        let map_builder = MapBuilder::new(&mut rng, floor_texture, wall_texture);

        ecs.insert_resource(map_builder.map);
        ecs.insert_resource(Camera::new(map_builder.player_start));
        ecs.insert_resource(texture_store);
        ecs.insert_resource(FrameTime(0.0));

        spawn_player(&mut ecs, map_builder.player_start);
        
        /* map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.centre())
            .for_each(|pos| spawn_enemy(&mut ecs, pos, &mut rng)); */

        Self {
            ecs,
            systems: build_scheduler(),
            controller: Controller::new(),
        }
    }
    fn tick(&mut self) {
        self.controller.update(); // TODO Move to ecs
        self.ecs.insert_resource(self.controller.button_state);
        //self.ecs.insert_resource(FrameTime(self.ecs.get_resource::<FrameTime>().unwrap().0 + get_frame_time()));
        self.systems.run(&mut self.ecs);

        //draw_circle(200.0, 700.0, 90.0, VIOLET);
        //draw_texture(&self.resources.get::<TextureStore>().unwrap().map_render.texture, VIEWPORT_X, VIEWPORT_Y, WHITE);
        self.controller.draw(); // TODO Move to ecs
    }
}

#[macroquad::main("Dungeon Crawl")]
async fn main() {
    let texture_store = TextureStore {
        player: load_texture("resources/Player.png")
            .await
            .expect("Player texture"),

        goblin: load_texture("resources/Goblin.png")
            .await
            .expect("Goblin texture"),
        giant: load_texture("resources/Giant.png")
            .await
            .expect("Giant texture"),
        twoheads: load_texture("resources/Twoheads.png")
            .await
            .expect("Twoheads texture"),
        warrior: load_texture("resources/Warrior.png")
            .await
            .expect("Warrior texture"),
        map_render: Texture2D::empty(),
        entity_render: Texture2D::empty(),
    };

    let floor_texture = load_texture("resources/Floor.png")
        .await
        .expect("Floor texture");
    let wall_texture = load_texture("resources/Wall.png")
        .await
        .expect("Wall texture");

    let mut game = Game::new(texture_store, floor_texture, wall_texture);
    let mut frame_time = 0.0;

    loop {
        clear_background(SKYBLUE);
        //draw_text("Hello", 100_f32, 200_f32, 12_f32, SKYBLUE);
        //game.update();

        /* if frame_time > 0.1 {
            game.tick();
            //draw_rectangle(20.0, 300.0, 300.0, 400.0, WHITE);
        } else {
            frame_time += get_frame_time();
        } */

        game.tick();
    }
}
