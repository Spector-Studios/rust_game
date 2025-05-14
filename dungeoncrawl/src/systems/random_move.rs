use crate::{events::WantsToMove, prelude::*};

pub fn random_move_system(
    //commands: Commands,
    //map: Res<Map>,
    mut writer: EventWriter<WantsToMove>,
    mut query: Query<(Entity, &TilePoint), With<MovesRandomly>>,
) {
    let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);

    const DIRECTIONS: [TilePoint; 4] = [
        TilePoint::new(0, 1),
        TilePoint::new(0, -1),
        TilePoint::new(1, 0),
        TilePoint::new(-1, 0),
    ];
    for (entity, mover_pos) in query.iter_mut() {
        let destination = rng.choice(DIRECTIONS).expect("Rng movement") + *mover_pos;

        writer.write(WantsToMove {
            entity,
            destination,
            is_player: false,
        });
    }
}
