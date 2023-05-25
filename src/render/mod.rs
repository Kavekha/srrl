use bevy::prelude::*;

pub mod tilemap_render;
use self::tilemap_render::spawn_map_render;

use crate::ecs_elements::GameState;




pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameMap), spawn_map_render);
    }
}
