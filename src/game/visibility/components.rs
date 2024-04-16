use bevy::prelude::*;

use crate::vectors::Vector2Int;


#[derive(Event)]
pub struct ComputeFovEvent;

#[derive(Component)]
pub struct View {
    pub visible_tiles: Vec<Vector2Int>,
    pub range: i32,
}

#[derive(Debug)]
pub enum ChangeTileVisibilityStatus{
    Visible,
    Hidden
}

#[derive(Component, Debug)]
pub struct ChangeTileVisibility{
    pub new_status: ChangeTileVisibilityStatus,
    pub visibility: i32,
    pub hidden: i32
}
