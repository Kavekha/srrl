//======> Documentation 0.20q
/*

STIMULUS:
    Tuer PJ.
    Rester en Vie.



*/

use bevy::prelude::*;

pub mod components;
mod plan_systems;

use crate::game::{pieces::components::Npc, player::Player, tileboard::components::BoardPosition};

use self::{components::{CheckGoal, Frozen}, plan_systems::{planning_adjacent_enemy, planning_can_do_melee_attack, planning_can_do_ranged_attack, planning_can_move, planning_enemy_in_sight, planning_evaluate_actions, planning_evaluate_goals, planning_has_allies_nearby, planning_has_low_life}};

use super::{combat_system::components::IsDead, rules::NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN, ActionSet};



pub struct IaPlugin;

impl Plugin for IaPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, ignore_npc_out_of_game_range.in_set(ActionSet::Planning))
        .add_systems(Update,
            (
            planning_evaluate_goals,
            planning_enemy_in_sight,
            planning_can_do_ranged_attack,
            planning_adjacent_enemy,
            planning_can_do_melee_attack,
            planning_has_low_life,
            planning_has_allies_nearby,
            planning_can_move,
            planning_evaluate_actions,
            
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
