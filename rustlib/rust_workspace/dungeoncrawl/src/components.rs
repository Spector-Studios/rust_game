use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use bracket_pathfinding::prelude::Point;

use crate::prelude::*;

/* #[derive(Debug, PartialEq)]
pub enum EntityType {
    Player,

    Goblin,
    Giant,
    Twoheads,
    Warrior,

    Amulet,
} */

#[derive(Component, Debug)]
pub struct Player {
    pub map_level: u8,
}

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct Item;

#[derive(Component, Debug)]
pub struct AmuletOfYala;

#[derive(Component, Debug)]
pub struct MovesRandomly;

#[derive(Component, Debug)]
pub struct ChasePlayer;

#[derive(Component, Debug)]
pub struct ProvidesDungeonMap;

#[derive(Component, Debug)]
pub struct EntityName(pub String);

#[derive(Component, Debug)]
pub struct Carried(pub Entity);

#[derive(Component, Debug)]
pub struct SelectedItemIndex(pub usize);

impl Deref for SelectedItemIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SelectedItemIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// TODO Make the texture a Rect or something
#[derive(Component, Debug)]
pub struct Render {
    pub texture: EntityType,
}

#[derive(Component, Debug)]
pub struct Timer {
    pub time: f32,
}

#[derive(Component, Debug)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_stale: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_stale: true,
        }
    }

    pub fn clone_stale(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_stale: true,
        }
    }
}

// TODO Timer is no longer needed because of turn based gameplay
#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub pos: TilePoint,
    pub health: Health,
    pub render: Render,
    pub field_of_view: FieldOfView,
    pub timer: Timer,
}

impl PlayerBundle {
    pub fn new(pos: TilePoint) -> Self {
        Self {
            player: Player { map_level: 0 },
            pos,
            health: Health {
                current: 13,
                max: 20,
            },
            render: Render {
                texture: EntityType::Player,
            },
            field_of_view: FieldOfView::new(8),
            timer: Timer { time: 0.0 },
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
    pub field_of_view: FieldOfView,
    pub movement_behaviour: ChasePlayer,
}

#[derive(Bundle, Debug)]
pub struct AmuletBundle {
    pub item: Item,
    pub amulet_of_yala: AmuletOfYala,
    pub pos: TilePoint,
    pub name: EntityName,
    pub render: Render,
}
