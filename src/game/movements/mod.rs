use bevy::prelude::*;

pub mod movement_systems;
pub mod components;


use self::movement_systems::{entity_move_to, entity_want_to_move, on_want_to_move_event};


pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
             // 0.19b            
            .add_systems(Update, on_want_to_move_event) //.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, entity_want_to_move.after(on_want_to_move_event))   
            .add_systems(Update, entity_move_to.after(entity_want_to_move))  
            ;
    }
}


