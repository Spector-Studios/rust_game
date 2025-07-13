use bracket_pathfinding::prelude::field_of_view_set;

use crate::prelude::*;

pub fn fov(map: Res<Map>, mut views: Query<(&TilePoint, &mut FieldOfView)>) {
    views
        .iter_mut()
        .filter(|(_, fov)| fov.is_stale)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set((*pos).into(), fov.radius, &(*map));
            fov.is_stale = false
        });
}
