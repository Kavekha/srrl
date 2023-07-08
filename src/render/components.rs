use std::collections::VecDeque;

use bevy::prelude::*;


#[derive(Component)]
pub struct GameCursorRender;

#[derive(Component)]
pub struct GameMapRender;

#[derive(Component)]
pub struct PathAnimator {
    pub path:VecDeque<Vec3>,
    pub wait_anim: bool
}

#[derive(Component)]
pub struct GameInterface;