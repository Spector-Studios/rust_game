use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: TilePoint, texture: Rect) {
    ecs.spawn(PlayerBundle {
        player: Player,
        pos,
        render: Render {
            texture_source: texture,
        },
        timer: Timer { time: 0.0 },
    });
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut Rng, pos: TilePoint, textures: [Rect; 4]) {
    ecs.spawn(EnemyBundle {
        enemy: Enemy,
        pos,
        render: Render {
            texture_source: match rng.u8(1..=4) {
                1 => textures[0],
                2 => textures[1],
                3 => textures[2],
                _ => textures[3],
            },
        }, // TODO random generation
    });
}
