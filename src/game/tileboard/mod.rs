use bevy::{prelude::*};

mod system_map;
pub mod components;

pub use components::Tile;

use self::{system_map::spawn_map};

use crate::states::GameState;



pub struct TileBoardPlugin;

impl Plugin for TileBoardPlugin {
    fn build(&self, app: &mut App){
        app
            // Init.
            .add_systems(OnEnter(GameState::Prerun), spawn_map)
            
            //SHOW_MAPGEN_VISUALIZER must be true.  //TODO : Broken lors de la division Logic VS Render.
            /* 
            .insert_resource(FixedTime::new_from_secs(FIXED_MAPGEN_NEW_SNAPSHOT))
            .add_systems(FixedUpdate, (
                display_map_generation, 
                despawn_screen::<GameMap>
            ).chain().run_if(
                in_state(GameState::MapGeneration)))       
            */
            ;  
    }
}