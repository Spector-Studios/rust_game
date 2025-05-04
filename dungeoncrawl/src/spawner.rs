use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: TilePoint, player_texture: Texture2D) {
    ecs.push((
        Player,
        pos,
        Render {
            texture: player_texture,
        },
    ));
}
