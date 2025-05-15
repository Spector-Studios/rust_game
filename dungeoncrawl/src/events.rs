use crate::prelude::*;

#[derive(Event)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: TilePoint,
    pub is_player: bool,
}

#[derive(Event)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
