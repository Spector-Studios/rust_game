use crate::prelude::*;

#[derive(Clone)]
pub struct FortressTheme;

impl FortressTheme {
    pub fn boxed_new() -> Box<dyn MapTheme> {
        Box::new(Self)
    }
}
impl MapTheme for FortressTheme {
    fn tile_to_render(&self, tile_type: TileType, _rng: &mut Rng) -> Rect {
        let (x, y) = match tile_type {
            TileType::Wall => (16.0, 16.0),
            TileType::Floor => (16.0, 0.0), // TileType::Floor => rng.choice([(0.0, 0.0), (0.0, 16.0), (16.0, 0.0)]).unwrap(),
        };

        Rect::new(x, y, 16.0, 16.0)
    }

    fn map_sheet_path(&self) -> String {
        String::from("map_fortress.png")
    }

    fn clone(&self) -> Box<dyn MapTheme> {
        Box::new(Self)
    }

    fn texture<'a>(&self, sprite_sheet: &'a SpriteSheet) -> &'a Texture2D {
        &sprite_sheet.map_fortress
    }
}

#[derive(Clone)]
pub struct ForestTheme;

impl ForestTheme {
    pub fn boxed_new() -> Box<dyn MapTheme> {
        Box::new(Self)
    }
}
impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType, _rng: &mut Rng) -> Rect {
        let (x, y) = match tile_type {
            TileType::Wall => (16.0, 16.0),
            TileType::Floor => (16.0, 0.0), // TileType::Floor => rng.choice([(0.0, 0.0), (0.0, 16.0), (16.0, 0.0)]).unwrap(),
        };

        Rect::new(x, y, 16.0, 16.0)
    }

    fn map_sheet_path(&self) -> String {
        String::from("map_forest.png")
    }

    fn clone(&self) -> Box<dyn MapTheme> {
        Box::new(Self)
    }

    fn texture<'a>(&self, sprite_sheet: &'a SpriteSheet) -> &'a Texture2D {
        &sprite_sheet.map_forest
    }
}
