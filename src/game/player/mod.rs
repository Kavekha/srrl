use bevy::prelude::*;

mod player_systems;
mod components;
pub mod cursor;

pub use components::Player;
pub use cursor::Cursor;

use self::{
    player_systems::{player_input, camera_follow, player_step_check, player_mouse_input}
};

use crate::{
    states::{GameState}
};

use super::combat::{components::CombatInfos, event_systems::create_action_infos};



pub struct PlayerPlugin;

//TODO : Input instead maybe? 
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app          
            .add_event::<PlayerInputReadyEvent>()
            .add_systems(Update, player_input.run_if(in_state(GameState::GameMap)))
            //.add_systems(Update, player_mouse_input.run_if(in_state(EngineState::PlayerInput)))
            .add_systems(Update, player_mouse_input.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, create_action_infos.run_if(resource_exists::<CombatInfos>()).after(player_mouse_input))
            
            .add_systems(Update, camera_follow.after(player_input).run_if(in_state(GameState::GameMap)))
            .add_systems(Update, player_step_check.run_if(in_state(GameState::GameMap)))
            ;
    }
}


#[derive(Event)]
pub struct PlayerInputReadyEvent;

#[derive(Event)]
pub struct PlayerActionEvent;
