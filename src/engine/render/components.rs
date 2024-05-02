use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::vectors::Vector2Int;

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Renderable {
    pub model: String,
}

#[derive(Component)]
pub struct GameCursorRender;

#[derive(Component)]
pub struct GameMapRender {
    pub floor : HashMap<Vector2Int, Entity>,
    pub wall : HashMap<Vector2Int, Entity>,
}


#[derive(Component)]
pub struct GameInterface;

#[derive(Component)]
pub struct AsciiText;
