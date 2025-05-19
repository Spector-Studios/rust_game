use bevy_time::TimerMode;
use std::time::Duration;

use crate::prelude::*;
use bevy_time::Timer;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum EntityType {
    Player,

    Goblin,
    Giant,
    Twoheads,
    Warrior,
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct MovesRandomly;

#[derive(Component, Debug)]
pub struct ChasePlayer;

#[derive(Component, Debug)]
pub struct EntityName(pub String);

// TODO Make the texture a Rect or something
#[derive(Component, Debug)]
pub struct Render {
    pub texture: EntityType,
}

#[derive(Debug)]
pub enum AnimationType {
    Move { from: TilePoint, to: TilePoint },
    Attack { from: TilePoint, to: TilePoint },
}

#[derive(Component, Debug)]
pub struct Animation {
    pub animation_type: AnimationType,
    pub timer: Timer,
}

impl Animation {
    pub fn new_movement(from: TilePoint, to: TilePoint) -> Self {
        Self {
            animation_type: AnimationType::Move { from, to },
            timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
        }
    }
}

#[derive(Component, Debug)]
pub struct InputTimer {
    pub time: f32,
}

#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub pos: TilePoint,
    pub health: Health,
    pub render: Render,
    pub timer: InputTimer,
}

impl PlayerBundle {
    pub fn new(pos: TilePoint) -> Self {
        Self {
            player: Player,
            pos,
            health: Health {
                current: 13,
                max: 20,
            },
            render: Render {
                texture: EntityType::Player,
            },
            timer: InputTimer { time: 0.0 },
        }
    }
}

// TODO Find a better way to store the behaviour
#[derive(Bundle, Debug)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub pos: TilePoint,
    pub name: EntityName,
    pub health: Health,
    pub render: Render,
    pub movement_behaviour: ChasePlayer,
}
