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

#[derive(Event)]
pub struct MoveEvent{
    pub entity: Entity,
    pub previous: Vector2Int,   // Où etait-il avant ce pas?
    pub next: Vector2Int,       // Où s'est-il rendu?
}