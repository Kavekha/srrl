use bevy::prelude::*;

pub mod system_map;
pub mod components;

use self::system_map::spawn_map;

use crate::game::states::GameState;

use super::manager::{MessageEvent, SpawnMapMessage};



pub struct TileBoardPlugin;

impl Plugin for TileBoardPlugin {
    fn build(&self, app: &mut App){
        app
            // Init.
            //.add_systems(OnEnter(GameState::Prerun), spawn_map) 
            .add_systems(OnEnter(GameState::Prerun), request_map_spawning)
            
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

fn request_map_spawning(
    mut ev_message: EventWriter<MessageEvent> 
){
    println!("Requested: Spawn Map!");
    ev_message.send(MessageEvent(Box::new(SpawnMapMessage)));
}