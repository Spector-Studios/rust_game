use crate::prelude::*;

#[derive(Resource)]
pub struct SpriteSheet {
    pub sprites: Texture2D,
}

impl SpriteSheet {
    pub async fn default() -> Result<Self> {
        let texture = load_texture("sprites.png").await?;

        Ok(Self { sprites: texture })
    }
}
