mod core;
use core::App;
use macroquad::prelude::*;
use macroquad::window;

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::file::set_pc_assets_folder("assets");

    let mut app = App::new();

    loop {
        clear_background(BLUE);
        draw_text("hello", 100.0, 400.0, 120.0, SKYBLUE);

        app.update();
        app.draw();

        next_frame().await
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "TEST TITLE 1".to_owned(),
        //high_dpi: true,
        ..Default::default()
    }
}
