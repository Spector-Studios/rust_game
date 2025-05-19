use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct AnimationQueue {
    pub queue: VecDeque<Entity>,
}
