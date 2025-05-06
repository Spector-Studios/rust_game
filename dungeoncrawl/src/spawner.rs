use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: TilePoint, texture: Texture2D) {
    ecs.spawn(PlayerBundle {
        player: Player,
        pos,
        render: Render {
            texture: EntityType::Player,
        },
        timer: Timer { time: 0.0 },
    });
}

pub fn spawn_enemy(ecs: &mut World, pos: TilePoint, texture: Texture2D, rng: &mut Rng) {
    ecs.spawn(EnemyBundle {
        enemy: Enemy,
        pos,
        render: Render {
            texture: EntityType::Goblin,
        }, // TODO random generation
    });
}
