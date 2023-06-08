use bevy::prelude::*;

use crate::{vectors::Vector2Int, states::GameState, despawn_screen, globals::{TILE_WIDTH_HALF, TILE_HEIGHT_HALF, STANDARD_TILE_WIDTH, STANDARD_TILE_HEIGHT}, render::{get_world_position, components::GameCursorRender}};


pub struct CursorPlugin;

impl Plugin for CursorPlugin{
    fn build(&self, app: &mut App) {
        app 
            .insert_resource(Cursor{grid_position:Vector2Int{x:0,y:0},world_position:Vec3::new(0.0, 0.0, 0.0)}) 
            .add_systems(Update, cursor_position.run_if(in_state(GameState::GameMap)))
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<GameCursorRender>);  
    }
}

#[derive(Resource, Component)]
pub struct Cursor {    
    pub grid_position: Vector2Int,
    pub world_position: Vec3
}

pub fn cursor_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut res_cursor: ResMut<Cursor>,
    window_query: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    for event in cursor_moved_events.iter() {
        println!("Cursor moved");
        let Ok((camera, camera_transform)) = camera_q.get_single() else { return };

        if let Some(world_position) = window_query.single().cursor_position() 
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())      
        {
                res_cursor.world_position = Vec3 {x:world_position.x, y:world_position.y, z:0.0};
                res_cursor.grid_position = get_grid_position(world_position.x, world_position.y);
                println!("Cursor updated");
        }
        //println!("Cursor: World position is : {:?}, Grid position is : {:?}", res_cursor.world_position, res_cursor.grid_position);
        //println!("Cursor: Sanity check: get world position is: {:?}", get_world_position(& res_cursor.grid_position));
    }
}

///map.x = (screen.x / TILE_WIDTH_HALF + screen.y / TILE_HEIGHT_HALF) /2;
///map.y = (screen.y / TILE_HEIGHT_HALF -(screen.x / TILE_WIDTH_HALF)) /2;
pub fn get_grid_position(
    x: f32,
    y: f32
) -> Vector2Int {
    //println!("GetGridPosition: {:?}", (x, y));
    // We need to reverse the numbers.
    let mut world_x = x;
    let mut world_y = y - (y * 2.0);
    //println!("GetGridPosition, reverse: {:?}", (world_x, world_y));
    // We also need to add half tile for each because...?? Trust me... //TODO : Understand why... -_-
    world_y += TILE_HEIGHT_HALF as f32;

    let grid_x_floor = (world_x / TILE_WIDTH_HALF as f32 + world_y / TILE_HEIGHT_HALF as f32).floor() / 2.0;
    let grid_y_floor = (world_y / TILE_HEIGHT_HALF as f32 - (world_x/ TILE_WIDTH_HALF as f32).floor()) / 2.0;



    Vector2Int{x:grid_x_floor as i32, y:grid_y_floor as i32}
}
