use std::collections::VecDeque;

use bevy::prelude::*;

use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct WantToMove {
    pub entity: Entity,
    pub path: VecDeque<Vector2Int>,
    pub target: Option<Vector2Int>,     // If Some, essaye d'attaquer ce qui se trouve à cette position.
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

// 0.20l : interrompre un deplacement du joueur s'il voit un NPC.
#[derive(Event)]
pub struct CancelMoveEvent{
    pub entity: Entity
}


// 0.20t : Indique les pas faits pendant le tour.
#[derive(Component)]
pub struct HasMoved{
    pub visited_tiles: Vec<Vector2Int>,
}