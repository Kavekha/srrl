use bevy::prelude::*;

mod player_systems;
mod player_inputs;
mod components;
pub mod cursor;

pub use components::Player;
pub use cursor::Cursor;

use self::{components::OnClickEvent, player_inputs::{combat_input, ig_call_menu_input, ig_inside_menu_input, mouse_scroll, on_click_action, player_choose_action_input, player_mouse_input}, player_systems::{camera_smooth_follow, exit_step_check}};

use crate::game::states::GameState;

use super::combat::CombatSet;


 


pub struct PlayerPlugin;

//TODO : Input instead maybe? 
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app          
            .add_event::<PlayerInputReadyEvent>()
            .add_event::<OnClickEvent>()               // Joueur clique: Attaque ou mouvement?    
            
            .add_systems(Update, player_mouse_input.run_if(in_state(GameState::Running)))   

            .add_systems(Update, combat_input.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, on_click_action.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic).after(combat_input))
            .add_systems(Update, player_choose_action_input.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic)) 

            .add_systems(Update, ig_call_menu_input.run_if(in_state(GameState::Running)))   // Appeler IG Menu si In Game.            
            .add_systems(Update, ig_inside_menu_input.run_if(in_state(GameState::Unavailable)))     // TODO : Put the game In Unavailable quand Menu Open 
            
            //.add_systems(Update, camera_follow.run_if(in_state(GameState::Running)))
            .add_systems(Update, camera_smooth_follow.run_if(in_state(GameState::Running)))
            
            .add_systems(Update, exit_step_check.run_if(in_state(GameState::Running)))

            .add_systems(Update, mouse_scroll)
            ;
    }
}


#[derive(Event)]
pub struct PlayerInputReadyEvent;

#[derive(Event)]
pub struct PlayerActionEvent;
