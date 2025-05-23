use crate::prelude::*;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum EntityType {
    Player,

    Goblin,
    Giant,
    Twoheads,
    Warrior,

    Amulet,
}

#[derive(Component, Debug)]
pub struct Player;

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
pub struct EntityName(pub String);

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
    pub timer: Timer,
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
