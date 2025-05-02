mod camera;
mod map;
mod map_builder;
mod player;
mod prelude;

use input_lib::Controller;
use crate::prelude::*;

struct Game {
    map: Map,
    player: Player,
    camera: Camera,
    controller: Controller,
}

impl Game {
    fn new(player_texture: Texture2D) -> Self {
        //let mut rng = RandGenerator::new();
        let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(player_texture),
            controller: Controller::new(),
        }
    }

    fn update(&mut self) {
        self.controller.update();
        self.player.update(
            self.controller.button_state,
            &self.map,
            //&mut self.view_area,
        );
        self.camera.update(&self.player);
    }

    fn render(&self) {
        //self.map.render(&self.view_area);
        //self.player.render(&self.view_area);
        self.camera.render(&self.map, &self.player);
        self.controller.draw();
    }
}

#[macroquad::main("Dungeon Crawl")]
async fn main() {
    #[cfg(debug_assertions)]
    rand::srand(2347382);
    
    #[cfg(not(debug_assertions))]
    rand::srand(macroquad::miniquad::date::now() as _);

    let player_texture = load_texture("resources/Player.png").await.expect("Texture");
    
    let mut game = Game::new(player_texture);
    let mut frame_time = 0.0;

    loop {
        clear_background(BLACK);
        //draw_text("Hello", 100_f32, 200_f32, 12_f32, SKYBLUE);
        game.update();

        if frame_time > (1.0 / FRAME_RATE) {
            game.render();
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
