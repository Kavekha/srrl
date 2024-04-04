use std::collections::VecDeque;

use bevy::prelude::*;

use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct WantToMove {
    pub entity: Entity,
    pub path: VecDeque<Vector2Int>,
    pub target: Option<Vector2Int>,
}

#[derive(Component)]
pub struct MoveTo{
    pub path: VecDeque<Vector2Int>,
    pub target: Option<Vector2Int>,
}