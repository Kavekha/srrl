use bevy::prelude::*;

mod player_systems;
mod components;
pub mod cursor;

pub use components::Player;
pub use cursor::Cursor;

use self::player_systems::{exit_step_check, player_mouse_input, camera_follow};

use crate::game::states::GameState;

 


pub struct PlayerPlugin;

//TODO : Input instead maybe? 
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app          
            .add_event::<PlayerInputReadyEvent>()
            .add_systems(Update, player_mouse_input.run_if(in_state(GameState::GameMap)))            
            .add_systems(Update, camera_follow.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, exit_step_check.run_if(in_state(GameState::GameMap)))
            ;
    }
}


#[derive(Event)]
pub struct PlayerInputReadyEvent;

#[derive(Event)]
pub struct PlayerActionEvent;
