use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{
    map_builders::TileType,
    vectors::Vector2Int
};


#[derive(Component, Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GridPosition{
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Tile {
    pub tiletype: TileType,
}

#[derive(Component)]
pub struct GameMap;

#[derive(Component, Default, Debug, Clone, Copy)]   //add Serialize, Deserialize, TODO
pub struct BoardPosition{
    pub v: Vector2Int
}