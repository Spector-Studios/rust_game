use strum_macros::{AsRefStr, EnumIter};

use crate::prelude::*;

#[derive(EnumIter, AsRefStr, Debug, Hash, PartialEq, Eq)]
// #[strum(suffix = ".png")] INFO Waiting for #440 on strum
pub enum EntityType {
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

impl EntityType {
    pub fn get_texture_source(&self) -> Rect {
        let (x, y) = match self {
            EntityType::Player => (16.0, 32.0),
            EntityType::Bat => (16.0, 0.0),
            EntityType::Cyclops => (0.0, 16.0),
            EntityType::Ghost => (16.0, 16.0),
            EntityType::Mage => (0.0, 32.0),
            EntityType::Amulet => (0.0, 0.0),
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
