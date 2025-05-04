pub use fastrand::Rng;
pub use input_lib::*;
pub use legion::systems::CommandBuffer;
pub use legion::world::SubWorld;
pub use legion::*;
pub use macroquad::prelude::*;

pub use crate::camera::Camera;
pub use crate::components::*;
pub use crate::map::*;
pub use crate::map_builder::*;
//pub use crate::player::*;
pub use crate::spawner::*;
pub use crate::systems::*;
pub use crate::texture_store::*;

pub const FRAME_RATE: f32 = 60.0;

pub const I_VIEWPORT_X: i32 = 15;
pub const I_VIEWPORT_Y: i32 = 300;
pub const VIEWPORT_WIDTH_T: i32 = 19;
pub const VIEWPORT_HEIGHT_T: i32 = 10;

pub const VIEWPORT_X: f32 = I_VIEWPORT_X as f32;
pub const VIEWPORT_Y: f32 = I_VIEWPORT_Y as f32;
pub const VIEWPORT_WIDTH: f32 = VIEWPORT_WIDTH_T as f32 * TILE_SIZE;
pub const VIEWPORT_HEIGHT: f32 = VIEWPORT_HEIGHT_T as f32 * TILE_SIZE;

pub const TILE_SIZE: f32 = 50.0;

pub const TILE_MAP_WIDTH: i32 = 80;
pub const TILE_MAP_HEIGHT: i32 = 50;
pub const NUM_TILES: usize = (TILE_MAP_WIDTH * TILE_MAP_HEIGHT) as usize;

pub const D_UP: TilePoint = TilePoint { x: 0, y: 1 };
pub const D_DOWN: TilePoint = TilePoint { x: 0, y: -1 };
pub const D_LEFT: TilePoint = TilePoint { x: -1, y: 0 };
pub const D_RIGHT: TilePoint = TilePoint { x: 1, y: 0 };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TilePoint {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add for TilePoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for TilePoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign for TilePoint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl TilePoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct TileRect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl TileRect {
    pub fn with_size(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn with_corners(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn shift(&mut self, delta: TilePoint) {
        self.x1 += delta.x;
        self.x2 += delta.x;
        self.y1 += delta.y;
        self.y2 += delta.y
    }

    pub fn intersects(&self, other: &TileRect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y2
    }

    pub fn centre(&self) -> TilePoint {
        TilePoint::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn for_each<T: FnMut(TilePoint)>(&self, mut f: T) {
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                f(TilePoint::new(x, y));
            }
        }
    }
}
