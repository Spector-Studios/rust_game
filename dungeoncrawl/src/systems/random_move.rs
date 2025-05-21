use crate::{
    events::{WantsToAttack, WantsToMove},
    prelude::*,
};

pub fn random_move_system(
    mut move_writer: EventWriter<WantsToMove>,
    mut attack_writer: EventWriter<WantsToAttack>,
    mut random_move_query: Query<(Entity, &TilePoint), With<MovesRandomly>>,
    entity_pos_query: Query<(Entity, &TilePoint, Option<&Player>)>,
) {
    // TODO Make Rng a resource
    let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);

    const DIRECTIONS: [TilePoint; 4] = [
        TilePoint::new(0, 1),
        TilePoint::new(0, -1),
        TilePoint::new(1, 0),
        TilePoint::new(-1, 0),
    ];
    for (enemy_entity, mover_pos) in random_move_query.iter_mut() {
        let destination = rng.choice(DIRECTIONS).expect("Rng movement") + *mover_pos;
        let mut attacked = false;

        entity_pos_query
            .iter()
            .filter(|(_, entity_pos, _)| **entity_pos == destination)
            .for_each(|(victim_entity, _, option_player)| {
                if option_player.is_some() {
                    attack_writer.write(WantsToAttack {
                        attacker: enemy_entity,
                        victim: victim_entity,
                    });
                }
                attacked = true;
            });

        if !attacked {
            move_writer.write(WantsToMove {
                entity: enemy_entity,
                destination,
                is_player: false,
            });
        }
    }
}
