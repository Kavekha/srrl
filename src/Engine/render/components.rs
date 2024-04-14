use bevy::prelude::*;


#[derive(Component)]
pub struct GameCursorRender;

#[derive(Component)]
pub struct GameMapRender;


#[derive(Component)]
pub struct GameInterface;

#[derive(Component)]
pub struct AsciiText;

// 0.20.a
#[derive(Component)]
pub struct TileRender { pub logic_entity: Entity }

#[derive(Component)]
pub struct TileRendered { pub render_entity: Entity }