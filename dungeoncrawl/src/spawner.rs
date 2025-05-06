use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: TilePoint) {
    ecs.spawn(PlayerBundle {
        player: Player,
        pos,
        render: Render {
            texture: EntityTexture::Player,
        },
        timer: Timer { time: 0.0 },
    });
}

pub fn spawn_enemy(ecs: &mut World, pos: TilePoint, rng: &mut Rng) {
    ecs.spawn(EnemyBundle {
        enemy: Enemy,
        pos,
        render: Render {
            texture: match rng.u8(1..=4) {
                1 => EntityTexture::Goblin,
                2 => EntityTexture::Giant,
                3 => EntityTexture::Twoheads,
                _ => EntityTexture::Warrior,
            },
        }, // TODO random generation
    });
}
