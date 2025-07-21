pub use bevy_ecs::prelude::*;
use bracket_pathfinding::prelude::Point;
pub use fastrand::Rng;
pub use input_lib::*;
pub use macroquad::prelude::*;
use std::collections::HashSet;

pub use crate::components::*;
pub use crate::map::*;
pub use crate::map_builder::*;
pub use crate::spawner::*;
pub use crate::systems::*;
pub use crate::texture_store::*;
pub use crate::viewport::Viewport;

//pub const FRAME_RATE: f32 = 30.0;

//pub const I_VIEWPORT_X: i32 = 15;
//pub const I_VIEWPORT_Y: i32 = 300;
pub const VIEWPORT_WIDTH_T: i32 = 21;
pub const VIEWPORT_HEIGHT_T: i32 = 11;

//pub static mut VIEWPORT_X: f32 = 0.0;
//pub static mut VIEWPORT_Y: f32 = 0.0;
pub const VIEWPORT_WIDTH: f32 = (VIEWPORT_WIDTH_T + 2) as f32 * TILE_SIZE;
pub const VIEWPORT_HEIGHT: f32 = (VIEWPORT_HEIGHT_T + 2) as f32 * TILE_SIZE;

pub const TILE_SIZE: f32 = 40.0;

pub const TILE_MAP_WIDTH: i32 = 70;
pub const TILE_MAP_HEIGHT: i32 = 40;
pub const NUM_TILES: usize = (TILE_MAP_WIDTH * TILE_MAP_HEIGHT) as usize;

pub const D_UP: TilePoint = TilePoint::new(0, 1);
pub const D_DOWN: TilePoint = TilePoint::new(0, -1);
pub const D_LEFT: TilePoint = TilePoint::new(-1, 0);
pub const D_RIGHT: TilePoint = TilePoint::new(1, 0);

pub const DIRECTIONS: [TilePoint; 4] = [D_UP, D_DOWN, D_LEFT, D_RIGHT];

pub const DEST_SIZE: Option<Vec2> = Some(vec2(TILE_SIZE, TILE_SIZE));

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::convert::From<Point> for TilePoint {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl std::convert::From<TilePoint> for Point {
    fn from(val: TilePoint) -> Self {
        Point { x: val.x, y: val.y }
    }
}

#[derive(Debug, Clone)]
pub struct TileRect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[allow(dead_code)]
impl TileRect {
    pub fn with_size(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: (x + w + 1),
            y2: (y + h + 1),
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
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
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

    pub fn point_set(&self) -> HashSet<TilePoint> {
        let mut result = HashSet::new();
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                result.insert(TilePoint::new(x, y));
            }
        }

        result
    }

    pub fn contains(&self, point: TilePoint) -> bool {
        (self.x1 <= point.x) && (self.x2 >= point.x) && (self.y1 <= point.y) && (self.y2 >= point.y)
    }
}

pub fn draw_text_centered(
    text: &str,
    x: f32,
    y: f32,
    color: Color,
    font: Option<&Font>,
    font_size: u16,
    rotation: f32,
) {
    let centre = get_text_center(text, font, font_size, 1.0, rotation);
    draw_text_ex(
        text,
        x - centre.x,
        y - centre.y,
        TextParams {
            font,
            font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation,
            color,
        },
    );
}
