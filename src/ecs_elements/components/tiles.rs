use bevy::prelude::*;

use crate::map_builders::commons::TileType;

#[derive(Component)]
pub struct GridPosition{
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Tile {
    pub tiletype: TileType,
}


#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct TileExit;

#[derive(Component)]
pub struct GameMap;

#[derive(Component)]
pub struct GameMapRender;