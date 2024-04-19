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
pub enum ChangeVisibilityStatus{
    Visible,
    Hidden
}

#[derive(Component, Debug)]
pub struct ChangeVisibility{
    pub new_status: ChangeVisibilityStatus
}

#[derive(Event)]

pub struct OutOfSightEvent{
    pub entity: Entity
}

#[derive(Component)]
pub struct Marked{
    pub marker_id: Entity 
}

#[derive(Component)]
pub struct Marker{
    pub marked_id: Entity
}

#[derive(Event)]
pub struct HasBeenSeenEvent {
    pub entity : Entity,
    pub saw_by : Entity
}

#[derive(Component)]
pub struct RenderVisibilityTile {
    pub visibility_score: i32
}
