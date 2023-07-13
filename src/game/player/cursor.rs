use bevy::prelude::*;

use crate::{vectors::Vector2Int, states::GameState, globals::STANDARD_TILE_SIZE, game::tileboard::components::BoardPosition};

use super::Player;


pub struct CursorPlugin;

impl Plugin for CursorPlugin{
    fn build(&self, app: &mut App) {
        app 
            .insert_resource(Cursor{grid_position:Vector2Int{x:0,y:0},world_position:Vec3::new(0.0, 0.0, 0.0), screen_position: None}) 
            .add_systems(Update, cursor_position.run_if(in_state(GameState::GameMap)))
        ;
    }
}


//camera.logical_viewport_size() donne la taille de l'ecran en pixel, de 0 à +X, et de 0 à +Y.
#[derive(Resource, Component)]
pub struct Cursor {    
    pub grid_position: Vector2Int,
    pub world_position: Vec3,
    pub screen_position: Option<Vec2>       // OptionVec2.
}

pub fn cursor_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut res_cursor: ResMut<Cursor>,
    window_query: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    query_player_pos: Query<&BoardPosition, With<Player>>,
) {
    for _event in cursor_moved_events.iter() {
        let Ok((camera, camera_transform)) = camera_q.get_single() else { return };

        if let Some(world_position) = window_query.single().cursor_position() 
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())      
        {
                res_cursor.world_position = Vec3 {x:world_position.x, y:world_position.y, z:0.0};
                res_cursor.grid_position = get_grid_position(world_position.x, world_position.y);
        }
        res_cursor.screen_position = camera.world_to_viewport(camera_transform, res_cursor.world_position);
        println!("Cursor: World position is : {:?}, Grid position is : {:?}", res_cursor.world_position, res_cursor.grid_position);
        let Ok(player_position) = query_player_pos.get_single() else { return };
        println!("Player Grid Position : {:?}", player_position.v);
        //println!("Cursor: Sanity check: get world position is: {:?}", get_world_position(& res_cursor.grid_position));
    }
}

///map.x = (screen.x / TILE_WIDTH_HALF + screen.y / TILE_HEIGHT_HALF) /2;
///map.y = (screen.y / TILE_HEIGHT_HALF -(screen.x / TILE_WIDTH_HALF)) /2;
pub fn get_grid_position(
    x: f32,
    y: f32
) -> Vector2Int {
    let mut grid_x = x;
    let mut grid_y= y - (y * 2.0);
    grid_x +=  (STANDARD_TILE_SIZE / 2) as f32;
    grid_y += (STANDARD_TILE_SIZE / 2) as f32;

    grid_x = grid_x / STANDARD_TILE_SIZE as f32;
    grid_y = grid_y / STANDARD_TILE_SIZE as f32;

    Vector2Int{x:grid_x as i32, y:grid_y as i32}

    /*
    //println!("GetGridPosition: {:?}", (x, y));
    // We need to reverse the numbers.
    let world_x = x;
    let world_y = y - (y * 2.0);

    let grid_x_floor = world_x / STANDARD_TILE_SIZE as f32;
    let grid_y_floor = world_y / STANDARD_TILE_SIZE as f32;

    Vector2Int{x:grid_x_floor as i32, y:grid_y_floor as i32}
     */
}
