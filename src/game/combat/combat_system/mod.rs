use bevy::prelude::*;

use self::intent_systems::{entity_get_hit, entity_miss_attack, entity_try_hit, entity_want_forfeit, entity_want_hit, on_event_entity_want_hit};

use super::ActionSet;

mod intent_systems;
pub mod components;

pub struct CombatSystemPlugin;

impl Plugin for CombatSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            // 0.20c : déplacé depuis combat.mod vers combat_systems.mod 
            .add_systems(Update, on_event_entity_want_hit.in_set(ActionSet::Planning)) 
            .add_systems(Update, entity_want_hit.in_set(ActionSet::Planning).after(on_event_entity_want_hit))
            .add_systems(Update, entity_want_forfeit.in_set(ActionSet::Planning))
            .add_systems(Update, entity_try_hit.in_set(ActionSet::Execute).after(entity_want_hit))
            .add_systems(Update, entity_miss_attack.in_set(ActionSet::Execute).after(entity_try_hit))
            .add_systems(Update, entity_get_hit.in_set(ActionSet::Execute).after(entity_try_hit))      
            ;
    }
}

