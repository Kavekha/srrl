use bevy::prelude::*;

use crate::game::states::GameState;

use self::npc_planning_systems::npc_planning;

use super::CombatSet;
pub mod npc_planning_systems;


pub struct IaPlugin;

impl Plugin for IaPlugin {
    fn build(&self, app: &mut App) {
        app
        // Plan NPC // Dans une partie IA?
        //.add_systems(Update, plan_action_forfeit.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, npc_planning.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
        
        ;
    }
}