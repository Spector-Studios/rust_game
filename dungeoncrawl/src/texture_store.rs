use fnv::FnvHashMap;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use crate::prelude::*;

#[derive(EnumIter, AsRefStr,Debug, Hash, PartialEq, Eq)]
// #[strum(suffix = ".png")] INFO Waiting for #440 on strum
pub enum SpriteKey {
    // Map
    Wall,
    Floor,

    // Player
    Player,

    // Enemies
    Bat,
    Cyclops,
    Ghost,
    Mage,

    // Items
    Amulet,
}

#[derive(Resource)]
pub struct SpriteSheet {
    pub sprites: FnvHashMap<SpriteKey, Texture2D>,
}

impl SpriteSheet {
    pub async fn default() -> Result<Self> {
        let mut sprites = FnvHashMap::default();

        for sprite_key in SpriteKey::iter() {
            let path = sprite_key.as_ref().to_lowercase() + ".png";
            sprites.insert(sprite_key, load_texture(&path).await?);
        }
        Ok(Self { sprites })
    }
}
