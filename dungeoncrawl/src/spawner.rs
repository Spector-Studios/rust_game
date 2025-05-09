use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: TilePoint) {
    ecs.spawn(PlayerBundle::new(pos));
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut Rng, pos: TilePoint) {
    ecs.spawn(EnemyBundle::random_enemy(pos, rng));
}
