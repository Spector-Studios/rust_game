use crate::prelude::*;

#[derive(Event)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: TilePoint,
}
