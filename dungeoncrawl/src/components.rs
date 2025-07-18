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
    /// Creates a movement animation from one tile to another with a 500ms duration.
    ///
    /// # Examples
    ///
    /// ```
    /// let anim = Animation::new_movement(TilePoint::new(1, 2), TilePoint::new(3, 4));
    /// if let AnimationType::Move { from, to } = anim.animation_type {
    ///     assert_eq!(from, TilePoint::new(1, 2));
    ///     assert_eq!(to, TilePoint::new(3, 4));
    /// }
    /// ```
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
    /// Creates a new player entity bundle with default health, render, and input timer values.
    ///
    /// The player is initialized at the specified tile position with 13 current health, 20 maximum health,
    /// the player texture, and an input timer set to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let position = TilePoint::new(5, 7);
    /// let player_bundle = PlayerBundle::new(position);
    /// assert_eq!(player_bundle.pos, position);
    /// assert_eq!(player_bundle.health.current, 13);
    /// ```
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
