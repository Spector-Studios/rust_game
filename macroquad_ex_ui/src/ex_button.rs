//use macroquad::{color::Color, input::touches, math::Rect, shapes::draw_rectangle};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct XButton {
    rect: Rect,
    label: String,
    color: Color,
    pub is_pressed: bool,
}

impl XButton {
    pub fn new(rect: Rect, label: &str, color: Color) -> Self {
        Self {
            rect,
            label: label.to_string(),
            color,
            is_pressed: false,
        }
    }

    pub fn update(&mut self) {
        self.is_pressed = touches()
            .iter()
            .any(|touch| self.rect.contains(touch.position));
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        );

        draw_text(self.label.as_str(), self.rect.x, self.rect.y, 30.0, BLUE);
    }
}
