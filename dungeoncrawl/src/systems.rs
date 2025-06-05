// TODO Do better organisation of these
pub mod chasing;
pub mod combat;
pub mod end_turn;
pub mod entity_render;
pub mod hud_render;
pub mod map_render;
pub mod movement;
pub mod player_input;
pub mod random_move;
pub mod update_pathfinding;

use crate::PathfindingMap;
use crate::prelude::*;
use crate::resources::EnemyQueue;
use bracket_pathfinding::prelude::Algorithm2D;
use std::collections::VecDeque;

pub fn setup_system(mut commands: Commands) {
    info!("setup start");
    commands.insert_resource(EnemyQueue(VecDeque::new()));

    let sprite_sheet = SpriteSheet {
        // TODO Use Bevy Asset Server
        sprites: Texture2D::from_image(
            &Image::from_file_with_format(
                include_bytes!("../assets/sprites.png"),
                Some(ImageFormat::Png),
            )
            .unwrap(),
        ),
    };

    commands.insert_resource(sprite_sheet);

    commands.insert_resource(Controller::new());
    commands.insert_resource(ButtonState::new());

    let mut rng = Rng::with_seed(macroquad::miniquad::date::now() as _);
    let map_builder = MapBuilder::new(&mut rng);

    let player_idx = map_builder
        .map
        .point2d_to_index(map_builder.player_start.into());

    spawn_player(&mut commands, map_builder.player_start);
    spawn_amulet(&mut commands, map_builder.amulet_start);

    map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|r| r.centre())
        .for_each(|pos| spawn_enemy(&mut commands, &mut rng, pos));

    commands.insert_resource(Viewport::new(map_builder.player_start));
    commands.insert_resource(PathfindingMap::new(&[player_idx], &map_builder.map));
    commands.insert_resource(map_builder.map);

    info!("setup end");
}

pub fn controller_update(mut controller: ResMut<Controller>, mut commands: Commands) {
    controller.update();
    controller.draw();
    commands.insert_resource(controller.button_state);
}
