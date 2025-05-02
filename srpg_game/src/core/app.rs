use crate::core::Game;
use input_lib::Controller;

use macroquad::prelude::*;

#[derive(Debug)]
enum AppState {
    Menu,
    Playing,
    End,
}

pub struct App {
    controller: Controller,
    game: Game,
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            controller: Controller::new(),
            game: Game::new(),
            state: AppState::Menu,
        }
    }

    fn draw_game_screen(&self) {
        //TODO
        let (x, y, w, h) = (10.0, 500.0, screen_width() - 20.0, 500.0);
        draw_text("hello", 20.0, screen_height() / 2.0, 12.0, BLUE);
        draw_rectangle_lines(x, y, w, h, 10.0, WHITE);
        draw_grid(10, 50.0, PINK, GREEN);
        //draw_circle(x + w / 2.0, y + h / 2.0, 100.0, BLUE);
    }
    fn draw_main_menu(&self) {
        let center = get_text_center("Welcome to flappy dragon", Option::None, 14, 1.0, 0.0);
        draw_text_ex(
            "Welcome to flappy dragon",
            center.x,
            screen_height() - center.y * 2.0,
            TextParams {
                font_size: 70,
                ..Default::default()
            },
        );
        draw_rectangle(200.0, self.game.x, 100.0, 100.0, WHITE);
    }

    fn draw_game_over(&self) {
        todo!()
    }
    pub fn update(&mut self) {
        self.controller.update();

        self.game.update(self.controller.button_state);
    }

    pub fn draw(&self) {
        self.draw_game_screen();
        match self.state {
            AppState::Menu => self.draw_main_menu(),
            AppState::Playing => self.game.draw(),
            AppState::End => self.draw_game_over(),
        }

        self.controller.draw();
    }
}
