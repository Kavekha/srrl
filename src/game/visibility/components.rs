use bevy::prelude::*;

use crate::vectors::Vector2Int;


#[derive(Event)]
pub struct ComputeFovEvent;

#[derive(Component)]
pub struct View {
    pub visible_tiles: Vec<Vector2Int>,
    pub range: i32,
}

pub enum ChangeTileVisibilityStatus{
    Visible,
    Hidden,
    HiddenButKnown,
}

#[derive(Component)]
pub struct ChangeTileVisibility{
    pub new_status: ChangeTileVisibilityStatus
}