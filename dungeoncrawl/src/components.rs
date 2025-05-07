use crate::prelude::*;
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
pub struct Render {
    pub texture: EntityType,
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct MovesRandomly;

#[derive(Component, Debug)]
pub struct Timer {
    pub time: f32,
}

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub pos: TilePoint,
    pub render: Render,
    pub timer: Timer,
}

#[derive(Bundle, Debug)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub pos: TilePoint,
    pub render: Render,
    pub moves_randomly: MovesRandomly,
}
