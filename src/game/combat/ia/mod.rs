//======> Documentation 0.20q
/*

STIMULUS:
    Tuer PJ.
    Rester en Vie.



*/

use bevy::prelude::*;

pub mod components;
mod ia_evaluate;
mod ia_planning;

use crate::game::{pieces::components::Npc, player::Player, tileboard::components::BoardPosition};

use self::{
    components::{CheckGoal, Frozen}, 
    ia_evaluate::{ia_evaluate_adjacent_enemy, ia_evaluate_allies_nearby, ia_evaluate_can_do_melee_attack, ia_evaluate_can_do_ranged_attack, ia_evaluate_can_move, ia_evaluate_check_target_knowledge, ia_evaluate_enemy_in_sight, ia_evaluate_goals, ia_evaluate_has_low_life, ia_evaluate_know_target_position, planning_actions}, ia_planning::{planning_approaching, planning_fleeing, planning_searching}};

use super::{combat_system::components::IsDead, rules::NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN, ActionSet};



pub struct IaPlugin;

impl Plugin for IaPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, ignore_npc_out_of_game_range.in_set(ActionSet::Planning))
        .add_systems(Update,
            (
                ia_evaluate_goals,
                ia_evaluate_check_target_knowledge,
                ia_evaluate_enemy_in_sight,
                ia_evaluate_know_target_position,
                ia_evaluate_can_do_ranged_attack,
                ia_evaluate_adjacent_enemy,
                ia_evaluate_can_do_melee_attack,
                ia_evaluate_has_low_life,
                ia_evaluate_allies_nearby,
                ia_evaluate_can_move,
            planning_actions,
            planning_approaching,
            planning_fleeing,
            planning_searching            
        )
        .chain()
        .in_set(ActionSet::Planning))
        ;
    }
}


fn ignore_npc_out_of_game_range(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, Option<&Frozen>), (With<Npc>, With<CheckGoal>, Without<IsDead>)>,
    position_q: Query<&BoardPosition>, 
    player_q: Query<Entity, With<Player>>,
){
    let Ok(player_entity) = player_q.get_single() else { return };
    let Ok(player_position) = position_q.get(player_entity) else { return };
    let mut to_remove_frozen = Vec::new();
    let mut to_remove_goal = Vec::new();

    for (npc_entity, npc_position, is_frozen) in npc_entity_fighter_q.iter() {
        if (player_position.v.x - npc_position.v.x).abs() > NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN 
        || (player_position.v.y - npc_position.v.y).abs() > NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN {
            //info!("NPC {:?} at {:?} is too far from player ({:?})", npc_entity, npc_position, player_position);
            commands.entity(npc_entity).insert(Frozen);
            to_remove_goal.push(npc_entity);
        } else if is_frozen.is_some() {
            //info!("npc_entity {:?} is frozen.", npc_entity);
            to_remove_frozen.push(npc_entity)
        };
    };
    for entity in to_remove_frozen {
        //info!("Frozen status on {:?} is removed.", entity);
        commands.entity(entity).remove::<Frozen>();        
    }
    for entity in to_remove_goal {
        commands.entity(entity).remove::<CheckGoal>();        
    }
}
