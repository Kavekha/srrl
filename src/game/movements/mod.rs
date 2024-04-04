use bevy::prelude::*;

pub mod movement_systems;
pub mod components;


use self::movement_systems::{action_entity_move, action_entity_try_move, entity_move_to, entity_want_to_move, on_want_to_move_event};

use super::combat::events::{EntityMoveEvent, EntityTryMoveEvent};



pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EntityTryMoveEvent>()         // Tente deplacement: check si target ou simple mouvement.
            .add_event::<EntityMoveEvent>()            // Se deplace.
            // Check des actions demandées.
            .add_systems(Update, action_entity_try_move)    //.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, action_entity_move)    //.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(action_entity_try_move))
 
            // 0.19b            
            .add_systems(Update, on_want_to_move_event) //.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, entity_want_to_move.after(on_want_to_move_event))   
            .add_systems(Update, entity_move_to.after(entity_want_to_move))  
            ;
    }
}


