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
            texture: EntityType::Amulet,
        },
    });
}

pub fn spawn_healing_potion(ecs: &mut Commands, pos: TilePoint) {
    ecs.spawn((
        Item,
        pos,
        EntityName("Health Potion".to_string()),
        ProvidesHealing { amount: 6 },
        Render {
            texture: EntityType::HealthPotion,
        },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut Commands, pos: TilePoint) {
    ecs.spawn((
        Item,
        pos,
        EntityName("Mapper".to_string()),
        ProvidesDungeonMap,
        Render {
            texture: EntityType::Map,
        },
    ));
}

pub fn spawn_entity(ecs: &mut Commands, pos: TilePoint, rng: &mut Rng) {
    match rng.u8(0..6) {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_enemy(ecs, rng, pos),
    }
}

fn bat() -> (i32, String, EntityType) {
    (1, "Goblin".to_string(), EntityType::Bat)
}

fn giant() -> (i32, String, EntityType) {
    (2, "Giant".to_string(), EntityType::Cyclops)
}
