use crate::prelude::*;
use strum::IntoEnumIterator;
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
pub struct Type(pub EntityType);

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
                current: 20,
                max: 20,
            },
            render: Render {
                texture: EntityType::Player,
            },
            timer: Timer { time: 0.0 },
        }
    }
}

#[derive(Bundle, Debug)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub pos: TilePoint,
    pub health: Health,
    pub render: Render,
    pub moves_randomly: MovesRandomly,
}

impl EnemyBundle {
    pub fn random_enemy(pos: TilePoint, rng: &mut Rng) -> Self {
        Self {
            enemy: Enemy,
            pos,
            health: Health {
                current: 10,
                max: 10,
            },
            render: Render {
                texture: rng
                    .choice(
                        EntityType::iter()
                            .filter(|t| *t != EntityType::Player)
                            .collect::<Vec<EntityType>>(),
                    )
                    .expect("Rng"),
            },
            moves_randomly: MovesRandomly,
        }
    }
}
