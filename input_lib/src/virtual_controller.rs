use macroquad::prelude::*;
use macroquad_ex_ui::XButton;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Buttons {
    DPadUp = 0b0000_0001,
    DPadDown = 0b0000_0010,
    DPadLeft = 0b0000_0100,
    DPadRight = 0b0000_1000,
    Action = 0b0001_0000,
    Back = 0b0010_0000,
    Start = 0b0100_0000,
} // Add to Controller if adding new Buttons

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonState {
    pub dpad_x: i32,
    pub dpad_y: i32,
    pub action: bool,
    pub back: bool,
    pub start: bool,
}

impl ButtonState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dpad_x: 0,
            dpad_y: 0,
            action: false,
            back: false,
            start: false,
        }
    }

    fn reset(&mut self) {
        self.dpad_x = 0;
        self.dpad_y = 0;
        self.action = false;
        self.back = false;
        self.start = false;
    }

    fn set(&mut self, button: Buttons) {
        match button {
            Buttons::DPadUp => self.dpad_y = 1,
            Buttons::DPadDown => self.dpad_y = -1,
            Buttons::DPadLeft => self.dpad_x = -1,
            Buttons::DPadRight => self.dpad_x = 1,
            Buttons::Action => self.action = true,
            Buttons::Back => self.back = true,
            Buttons::Start => self.start = true,
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Controller {
    buttons: Vec<(XButton, Buttons)>,
    pub button_state: ButtonState,
}

impl Controller {
    #[must_use]
    pub fn new() -> Self {
        Self {
            buttons: vec![
                (
                    XButton::new(
                        Rect::new(200.0, screen_height() - 400.0, 100.0, 100.0),
                        "↑",
                        RED,
                    ),
                    Buttons::DPadUp,
                ),
                (
                    XButton::new(
                        Rect::new(200.0, screen_height() - 200.0, 100.0, 100.0),
                        "↓",
                        RED,
                    ),
                    Buttons::DPadDown,
                ),
                (
                    XButton::new(
                        Rect::new(100.0, screen_height() - 300.0, 100.0, 100.0),
                        "←",
                        RED,
                    ),
                    Buttons::DPadLeft,
                ),
                (
                    XButton::new(
                        Rect::new(300.0, screen_height() - 300.0, 100.0, 100.0),
                        "→",
                        RED,
                    ),
                    Buttons::DPadRight,
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() - 150.0,
                            screen_height() - 350.0,
                            100.0,
                            100.0,
                        ),
                        "A",
                        RED,
                    ),
                    Buttons::Action,
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() - 250.0,
                            screen_height() - 250.0,
                            100.0,
                            100.0,
                        ),
                        "B",
                        RED,
                    ),
                    Buttons::Back,
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() / 2.0 - 50.0,
                            screen_height() - 500.0,
                            100.0,
                            50.0,
                        ),
                        "Start",
                        RED,
                    ),
                    Buttons::Start,
                ),
            ],
            button_state: ButtonState::new(),
        }
    }

    pub fn update(&mut self) {
        self.button_state.reset();
        for (btn, flag) in &mut self.buttons {
            btn.update();
            if btn.is_pressed {
                self.button_state.set(*flag);
            }
        }
    }

    pub fn draw(&self) {
        for (btn, _) in &self.buttons {
            btn.draw();
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
