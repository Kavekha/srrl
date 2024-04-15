use bevy::{prelude::*, utils::HashMap};

use crate::vectors::Vector2Int;


#[derive(Event)]
pub struct ComputeFovEvent;

#[derive(Component)]
pub struct View {
    pub visible_tiles: Vec<Vector2Int>,
    pub range: i32,
}