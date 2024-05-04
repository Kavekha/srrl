use std::collections::VecDeque;

use bevy::prelude::*;

use crate::vectors::Vector2Int;

#[derive(Event)]
pub struct AnimateEvent {
    //anim_type?
    pub entity: Entity,
    pub path: VecDeque<Vector2Int>,
    pub wait_anim: bool
}
 
#[derive(Event)]
pub struct GraphicsWaitEvent;


#[derive(Component)]
pub struct PathAnimator {
    pub path:VecDeque<Vec3>,
    pub wait_anim: bool
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct RemoveEntity;


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
pub struct ShootingAnimator;