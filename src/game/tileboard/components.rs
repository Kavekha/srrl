use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{
    map_builders::TileType,
    vectors::Vector2Int
};


#[derive(Component)]
pub struct Tile {
    pub tiletype: TileType,
}

#[derive(Component)]
pub struct GameMap;

#[derive(Component, Default, Debug, Clone, Copy, Serialize, Deserialize)] 
pub struct BoardPosition{
    pub v: Vector2Int
}

#[derive(Component)]
pub struct ExitMapTile;

