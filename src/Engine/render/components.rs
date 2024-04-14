use bevy::{prelude::*, utils::HashMap};

use crate::vectors::Vector2Int;


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

// 0.20.a
#[derive(Component)]
pub struct TileRender { pub logic_entity: Entity }

#[derive(Component)]
pub struct TileRendered { pub render_entity: Entity }