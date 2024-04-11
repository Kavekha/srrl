use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::vectors::Vector2Int;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]


pub struct Player;

#[derive(Event)]
pub struct OnClickEvent {
    pub entity: Entity,
    pub tile: Vector2Int
}

#[derive(Event)]
pub struct WantToMoveEvent {
    pub entity: Entity,
    pub tile: Vector2Int
}

