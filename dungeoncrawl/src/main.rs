mod camera;
mod texture_store;
mod map;
mod map_builder;
mod player;
mod prelude;
mod components;
mod spawner;
mod systems;

use input_lib::Controller;
use crate::prelude::*;

struct Game {
    ecs: World,
    resources: Resources,
    systems: Schedule,
    //map: Map,
    //player: Player,
    //camera: Camera,
    controller: Controller,
}

impl Game {
    fn new(player_texture: Texture2D, wall_texture:Texture2D, floor_texture: Texture2D) -> Self {
        //let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        //let map_builder = MapBuilder::new(&mut rng, floor_texture);

        /* Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start,player_texture),
            camera: Camera::new(),
            controller: Controller::new(),
        } */

        let mut ecs = World::default();
        let mut resources = Resources::default();

        let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        let map_builder = MapBuilder::new(&mut rng, floor_texture, wall_texture);
        spawn_player(&mut ecs, map_builder.player_start, player_texture);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TextureStore::new());

        Self { ecs, resources, systems: build_scheduler(), controller: Controller::new() }
    }

    /* fn update(&mut self) {
        self.controller.update();
        self.player.update(
            self.controller.button_state,
            &self.map,
            //&mut self.view_area,
        );
        self.camera.update(&self.player);
    } */

    /* fn render(&self) {
        //self.map.render(&self.view_area);
        //self.player.render(&self.view_area);
        //self.camera.render(&self.map);
        self.map.render(&self.camera);
        self.player.render(&self.camera);
        self.controller.draw();
    } */

    fn tick(&mut self) {
        self.controller.update();
        self.resources.insert(self.controller.button_state);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        //draw_texture(&self.resources.get::<TextureStore>().unwrap().map_render.texture, VIEWPORT_X, VIEWPORT_Y, WHITE);
        self.controller.draw();
    }
}

#[macroquad::main("Dungeon Crawl")]
async fn main() {

    let player_texture = load_texture("resources/Player.png").await.expect("Player texture");
    let floor_texture = load_texture("resources/Floor.png").await.expect("Floor texture.");
    let wall_texture = load_texture("resources/Wall.png").await.expect("Wall texture.");

    let mut game = Game::new(player_texture, wall_texture, floor_texture);
    let mut frame_time = 0.0;

    loop {
        clear_background(SKYBLUE);
        //draw_text("Hello", 100_f32, 200_f32, 12_f32, SKYBLUE);
        //game.update();

        if frame_time > (1.0 / FRAME_RATE) {
            game.tick();
        } else {
            frame_time += get_frame_time();
        }
        draw_text(
            format!("{}", screen_width()).as_str(),
            100.0,
            200.0,
            100.0,
            SKYBLUE,
        );
        next_frame().await;
    }
}
