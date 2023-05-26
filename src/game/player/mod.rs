use bevy::prelude::*;

mod system_player;
mod components;

pub use components::{Monster, Player, Piece, Npc, Stats};

use self::{
    system_player::{player_input, camera_follow, player_step_check}
};

use crate::{states::GameState, despawn_screen};



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(GameState::NewGame), character_creation)              
            .add_systems(Update, player_input.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, camera_follow.after(player_input).run_if(in_state(GameState::GameMap)))
            .add_systems(Update, player_step_check.run_if(in_state(GameState::GameMap)))
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Player>);  
    }
}



