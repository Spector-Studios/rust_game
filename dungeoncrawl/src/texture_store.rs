// TODO May be move to prelude/texture_store.rs?
use crate::prelude::*;

#[derive(Debug)]
pub enum EntityTexture {
    Player,

    Goblin,
    Giant,
    Twoheads,
    Warrior,
}

#[derive(Resource, Debug, Clone)]
pub struct TextureStore {
    //pub floor: Texture2D,
    //pub wall: Texture2D,
    pub player: Texture2D,

    pub goblin: Texture2D,
    pub giant: Texture2D,
    pub twoheads: Texture2D,
    pub warrior: Texture2D,

    pub map_render: Texture2D,
    pub entity_render: Texture2D,
}
