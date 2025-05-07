use crate::prelude::*;

pub fn random_move_system(map: Res<Map>, mut query: Query<&mut TilePoint, With<MovesRandomly>>) {
    let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);

    const DIRECTIONS: [TilePoint; 4] = [
        TilePoint::new(0, 1),
        TilePoint::new(0, -1),
        TilePoint::new(1, 0),
        TilePoint::new(-1, 0),
    ];
    for mut mover_pos in query.iter_mut() {
        let destination = rng.choice(DIRECTIONS).expect("Rng movement") + *mover_pos;

        if map.can_enter_tile(destination) {
            *mover_pos = destination;
        }
    }
}
