// TODO May be move to prelude/texture_store.rs?
use crate::prelude::*;

pub struct TextureStore {
    //pub player_texture: Texture2D,
    //pub wall_texture: Texture2D,
    //pub floor_texture: Texture2D,
    pub map_render: RenderTarget,
    pub entity_render: RenderTarget,
}

impl TextureStore {
    pub fn new(//player_texture: Texture2D,
        //wall_texture: Texture2D,
        //floor_texture: Texture2D,
    ) -> Self {
        let map_render = render_target(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32);
        let entity_render = render_target(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32);

        map_render.texture.set_filter(FilterMode::Nearest);
        entity_render.texture.set_filter(FilterMode::Nearest);

        Self {
            //player_texture,
            //wall_texture,
            //floor_texture,
            map_render,
            entity_render,
        }
    }
}
