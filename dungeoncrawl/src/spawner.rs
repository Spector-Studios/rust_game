use crate::prelude::*;

pub fn spawn_player(ecs: &mut Commands, pos: TilePoint) {
    ecs.spawn(PlayerBundle::new(pos));
}

pub fn spawn_enemy(ecs: &mut Commands, rng: &mut Rng, pos: TilePoint) {
    let (hp, name, render_type) = match rng.i8(0..=10) {
        1..=7 => bat(),
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
        field_of_view: FieldOfView::new(6),
        movement_behaviour: ChasePlayer,
    });
}

pub fn spawn_amulet(ecs: &mut Commands, pos: TilePoint) {
    ecs.spawn(AmuletBundle {
        item: Item,
        amulet_of_yala: AmuletOfYala,
        pos,
        name: EntityName("Amulet".to_string()),
        render: Render {
            texture: SpriteKey::Amulet,
        },
    });
}

fn bat() -> (i32, String, SpriteKey) {
    (1, "Goblin".to_string(), SpriteKey::Bat)
}

fn giant() -> (i32, String, SpriteKey) {
    (2, "Giant".to_string(), SpriteKey::Cyclops)
}
