use bevy_ecs::prelude::*;
use enumset::EnumSet;
use enumset::EnumSetType;
use macroquad::prelude::*;
use macroquad_ex_ui::XButton;

#[derive(EnumSetType, Debug)]
pub enum Buttons {
    A,
    B,
    X,
    Y,
    Start,
    Select,
}

#[derive(Clone, Copy, Debug)]
pub enum DPadButtons {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonKind {
    DPad(DPadButtons),
    Action(Buttons),
}

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct ButtonState {
    pub dpad_x: i32,
    pub dpad_y: i32,
    pub buttons: EnumSet<Buttons>,
}

impl ButtonState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dpad_x: 0,
            dpad_y: 0,
            buttons: EnumSet::empty(),
        }
    }

    pub fn reset(&mut self) {
        self.dpad_x = 0;
        self.dpad_y = 0;
        self.buttons.clear();
    }

    fn set(&mut self, button: ButtonKind) {
        match button {
            ButtonKind::DPad(dpad_buttons) => match dpad_buttons {
                DPadButtons::Left => self.dpad_x -= 1,
                DPadButtons::Right => self.dpad_x += 1,
                DPadButtons::Up => self.dpad_y += 1,
                DPadButtons::Down => self.dpad_y -= 1,
            },
            ButtonKind::Action(buttons) => {
                self.buttons.insert(buttons);
            }
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Resource)]
pub struct Controller {
    buttons: [(XButton, ButtonKind); 10],
    pub button_state: ButtonState,
}

impl Controller {
    #[must_use]
    pub fn new() -> Self {
        Self {
            buttons: [
                (
                    XButton::new(
                        Rect::new(200.0, screen_height() - 400.0, 100.0, 100.0),
                        "↑",
                        RED,
                    ),
                    ButtonKind::DPad(DPadButtons::Up),
                ),
                (
                    XButton::new(
                        Rect::new(200.0, screen_height() - 200.0, 100.0, 100.0),
                        "↓",
                        RED,
                    ),
                    ButtonKind::DPad(DPadButtons::Down),
                ),
                (
                    XButton::new(
                        Rect::new(100.0, screen_height() - 300.0, 100.0, 100.0),
                        "←",
                        RED,
                    ),
                    ButtonKind::DPad(DPadButtons::Left),
                ),
                (
                    XButton::new(
                        Rect::new(300.0, screen_height() - 300.0, 100.0, 100.0),
                        "→",
                        RED,
                    ),
                    ButtonKind::DPad(DPadButtons::Right),
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() - 150.0 + 10.0,
                            screen_height() - 350.0 - 10.0,
                            100.0,
                            100.0,
                        ),
                        "A",
                        RED,
                    ),
                    ButtonKind::Action(Buttons::A),
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() - 250.0 - 10.0,
                            screen_height() - 250.0 + 10.0,
                            100.0,
                            100.0,
                        ),
                        "B",
                        RED,
                    ),
                    ButtonKind::Action(Buttons::B),
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() - 250.0 - 10.0,
                            screen_height() - 350.0 - 10.0,
                            100.0,
                            100.0,
                        ),
                        "X",
                        RED,
                    ),
                    ButtonKind::Action(Buttons::X),
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() - 150.0 + 10.0,
                            screen_height() - 250.0 + 10.0,
                            100.0,
                            100.0,
                        ),
                        "Y",
                        RED,
                    ),
                    ButtonKind::Action(Buttons::Y),
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() / 2.0 - 50.0 - 100.0,
                            screen_height() - 600.0,
                            100.0,
                            50.0,
                        ),
                        "Start",
                        RED,
                    ),
                    ButtonKind::Action(Buttons::Start),
                ),
                (
                    XButton::new(
                        Rect::new(
                            screen_width() / 2.0 - 50.0 + 100.0,
                            screen_height() - 600.0,
                            100.0,
                            50.0,
                        ),
                        "Select",
                        RED,
                    ),
                    ButtonKind::Action(Buttons::Select),
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

    pub fn draw(&self, font: Option<&Font>) {
        for (btn, _) in &self.buttons {
            btn.draw(font);
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
