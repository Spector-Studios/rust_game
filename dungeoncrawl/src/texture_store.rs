// TODO May be move to prelude/texture_store.rs?
use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct TextureStore {
    pub map_render: Texture2D,
    pub entity_render: Texture2D,
}

impl TextureStore {
    pub fn new() -> Self {
        //let map_render = render_target(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32);
        //let entity_render = render_target(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32);

        let map_render = Texture2D::empty();
        let entity_render = Texture2D::empty();

        map_render.set_filter(FilterMode::Nearest);
        entity_render.set_filter(FilterMode::Nearest);

        Self {
            map_render,
            entity_render,
        }
    }
}
