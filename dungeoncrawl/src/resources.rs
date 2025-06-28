use std::ops::Deref;

use bracket_pathfinding::prelude::DijkstraMap;

use crate::prelude::*;

#[derive(Resource)]
pub struct PathfindingMap {
    pub dijsktra_map: DijkstraMap,
    pub is_stale: bool,
}

impl PathfindingMap {
    pub fn new(search_targets: &[usize], map: &Map) -> Self {
        Self {
            dijsktra_map: DijkstraMap::new(
                TILE_MAP_WIDTH,
                TILE_MAP_HEIGHT,
                search_targets,
                map,
                1024.0,
            ),
            is_stale: false,
        }
    }
}

#[derive(Resource)]
pub struct Theme {
    pub theme: Box<dyn MapTheme>,
}

impl Theme {
    pub fn tile_to_render(&self, tile_type: TileType, rng: &mut Rng) -> Rect {
        self.theme.tile_to_render(tile_type, rng)
    }

    pub fn map_sheet_path(&self) -> String {
        self.theme.map_sheet_path()
    }

    pub fn texture<'a>(&self, sprite_sheet: &'a SpriteSheet) -> &'a Texture2D {
        self.theme.texture(sprite_sheet)
    }
}

#[derive(Resource)]
pub struct FontResource(pub Font);
impl Deref for FontResource {
    type Target = Font;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
