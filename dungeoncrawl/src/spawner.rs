use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: TilePoint) {
    ecs.spawn(PlayerBundle::new(pos));
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut Rng, pos: TilePoint) {
    let (hp, name, render_type) = match rng.i8(0..=10) {
        1..=7 => goblin(),
        _ => giant(),
    };
    ecs.spawn(EnemyBundle {
        enemy: Enemy,
        pos,
        name: EntityName(name),
        health: Health {
            current: hp,
            max: hp,
        },
        render: Render {
            texture: render_type,
        },
        movement_behaviour: ChasePlayer,
    });
}

fn goblin() -> (i32, String, EntityType) {
    (1, "Goblin".to_string(), EntityType::Goblin)
}

fn giant() -> (i32, String, EntityType) {
    (2, "Giant".to_string(), EntityType::Giant)
}
