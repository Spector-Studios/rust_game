use crate::prelude::*;
use serde::Deserialize;
use strum::AsRefStr;
use strum::EnumIter;

#[derive(Deserialize, EnumIter, AsRefStr, Debug, Hash, PartialEq, Eq, Clone, Copy)]
// #[strum(suffix = ".png")] INFO Waiting for #440 on strum
pub enum RenderKey {
    // Player
    Player,

    // Enemies
    Bat,
    Cyclops,
    Ghost,
    Mage,

    // Items
    Amulet,
    HealthPotion,
    Map,
}

impl RenderKey {
    pub fn get_texture_source(&self) -> Rect {
        let (x, y) = match self {
            RenderKey::Player => (16.0, 32.0),
            RenderKey::Bat => (16.0, 0.0),
            RenderKey::Cyclops => (0.0, 16.0),
            RenderKey::Ghost => (16.0, 16.0),
            RenderKey::Mage => (0.0, 32.0),
            RenderKey::Amulet => (0.0, 0.0),
            RenderKey::HealthPotion => (32.0, 0.0),
            RenderKey::Map => (32.0, 16.0),
        };

        Rect::new(x, y, 16.0, 16.0)
    }
}

#[derive(Resource)]
pub struct SpriteSheet {
    pub entities: Texture2D,
    pub map_fortress: Texture2D,
    pub map_forest: Texture2D,
}

impl SpriteSheet {
    pub async fn new() -> Self {
        let entities = load_texture("entities.png").await.unwrap();
        let map_fortress = load_texture("map_fortress.png").await.unwrap();
        let map_forest = load_texture("map_forest.png").await.unwrap();

        build_textures_atlas();

        Self {
            entities,
            map_fortress,
            map_forest,
        }
    }
}
