use std::collections::VecDeque;

use bevy::prelude::*;

use crate::vectors::Vector2Int;

#[derive(Event)]
pub struct AnimateEvent {
    //anim_type?
    pub entity: Entity,
    pub path: VecDeque<Vector2Int>
}
 
#[derive(Event)]
pub struct GraphicsWaitEvent;


#[derive(Component)]
pub struct PathAnimator {
    pub path:VecDeque<Vec3>,
    pub wait_anim: bool
}