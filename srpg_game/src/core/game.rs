use input_lib::ButtonState;

pub struct Game {
    pub x: f32,
}

impl Game {
    pub fn new() -> Self {
        Self { x: 0.0 }
    }

    pub fn update(&mut self, input: ButtonState) {
        //todo!()
    }

    pub fn draw(&self) {
        //todo!()
    }
}
