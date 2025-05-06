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

struct Game {
    // ecs: World,
    // systems: Schedule,
    controller: Controller,
}

impl Game {
    fn new(
        //sprite_sheet: SpritesStore,
        // player_texture: Rect,
        // enemy_textures: [Rect; 4],
        // floor_texture: Rect,
        // wall_texture: Rect,
    ) -> Self {
        // let mut ecs = World::default();

        // let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
        // let map_builder = MapBuilder::new(&mut rng, floor_texture, wall_texture);

        // ecs.insert_resource(map_builder.map);
        // ecs.insert_resource(Camera::new(map_builder.player_start));
        // ecs.insert_resource(sprite_sheet);
        // ecs.insert_resource(FrameTime(0.0));

        // spawn_player(&mut ecs, map_builder.player_start, player_texture);

        // map_builder
        //     .rooms
        //     .iter()
        //     .skip(1)
        //     .map(|r| r.centre())
        //     .for_each(|room_centre| spawn_enemy(&mut ecs, &mut rng, room_centre, enemy_textures));

        Self {
            // ecs,
            // systems: build_scheduler(),
            controller: Controller::new(),
        }
    }
    fn tick(&mut self) {
        self.controller.update(); // TODO Move to ecs
        //self.ecs.insert_resource(self.controller.button_state);
        //self.ecs.insert_resource(FrameTime(self.ecs.get_resource::<FrameTime>().unwrap().0 + get_frame_time()));
        //self.systems.run(&mut self.ecs);

        //draw_circle(200.0, 700.0, 90.0, VIOLET);
        //draw_texture(&self.resources.get::<TextureStore>().unwrap().map_render.texture, VIEWPORT_X, VIEWPORT_Y, WHITE);
        self.controller.draw(); // TODO Move to ecs
    }
}

#[macroquad::main("Dungeon Crawl")]
async fn main() {
    /* let player = load_texture("resources/Player.png")
        .await
        .expect("Player texture");

    let goblin = load_texture("resources/Goblin.png")
        .await
        .expect("Goblin texture");
    let giant = load_texture("resources/Giant.png")
        .await
        .expect("Giant texture");
    let twoheads = load_texture("resources/Twoheads.png")
        .await
        .expect("Twoheads texture");
    let warrior = load_texture("resources/Warrior.png")
        .await
        .expect("Warrior texture");
    let map_render = Texture2D::empty();
    let entity_render = Texture2D::empty();

    let floor_texture = load_texture("resources/Floor.png")
        .await
        .expect("Floor texture");
    let wall_texture = load_texture("resources/Wall.png")
        .await
        .expect("Wall texture");

    let enemy_textures = [goblin, giant, twoheads, warrior]; */

    //let sprites = load_texture("resources/sprites.png")
    //    .await
    //    .expect("Sprites");
    //let sprite_sheet = SpritesStore { sprites };

    // let player = Rect::new(96.0, 32.0, 32.0, 32.0);

    // let floor_texture = Rect::new(96.0, 0.0, 32.0, 32.0);
    // let wall_texture = Rect::new(64.0, 96.0, 32.0, 32.0);

    // let goblin = Rect::new(32.0, 32.0, 32.0, 32.0);
    // let giant = Rect::new(0.0, 0.0, 32.0, 32.0);
    // let twoheads = Rect::new(32.0, 96.0, 32.0, 32.0);
    // let warrior = Rect::new(96.0, 96.0, 32.0, 32.0);

    // let enemy_textures = [goblin, giant, twoheads, warrior];

    let mut game = Game::new(
        //sprite_sheet,
        // player,
        // enemy_textures,
        // floor_texture,
        // wall_texture,
    );
    let mut frame_time = 0.0;

    loop {
        clear_background(SKYBLUE);
        //draw_text("Hello", 100_f32, 200_f32, 12_f32, SKYBLUE);
        //game.update();

        if frame_time > (1.0/FRAME_RATE) {
            game.tick();
            //draw_rectangle(20.0, 300.0, 300.0, 400.0, WHITE);
        } else {
            frame_time += get_frame_time();
        }

        //game.tick();
    }
}
