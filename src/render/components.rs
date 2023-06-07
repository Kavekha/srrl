use std::collections::VecDeque;

use bevy::prelude::*;


#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct TileExit;


#[derive(Component)]
pub struct GameMapRender;

#[derive(Component)]
pub struct PathAnimator(pub VecDeque<Vec3>);
