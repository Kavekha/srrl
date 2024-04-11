//v0.19h
//https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://citeseerx.ist.psu.edu/document%3Frepid%3Drep1%26type%3Dpdf%26doi%3D012ef03d0f951092b8645b69aebdbce900ac03e4&ved=2ahUKEwingo_qkrKFAxWsTaQEHYTTAFIQFnoECCMQAQ&usg=AOvVaw3spa-hKcVtGhhaO5QmYsWT


use bevy::prelude::*;


use crate::game::{combat::{components::IsDead, events::Turn, ia::components::{Goal, GoalType, Planning}}, pieces::components::Npc, player::Player};

use super::components::CheckGoal;



// Donne à chaque NPC le but de tuer le joueur.
pub fn npc_initialise_goals(
    mut commands: Commands,
    entity_npc_q: Query<Entity, With<Npc>>,// Joué en Setup, pas besoin de verifier si vivant etcs.
    entity_player_q: Query<Entity, With<Player>>,  // Notre cible.
) {
    let Ok(player_entity) = entity_player_q.get_single() else { println!("WARNING: No player found for initialise goals"); return;};
    for npc_entity in entity_npc_q.iter() {
        commands.entity(npc_entity).insert(Goal { id: GoalType::KillEntity { id: player_entity}});
        println!("Kill Goal initialized for NPC {:?}", npc_entity);
    }
}

// 0.19h : Verifie si ce goal est tjrs d'actualité. A ce stade on ne fait rien de pluss, car on a qu'un seul goal. Sera utile plus tard. 
// A voir comment industrialiser la requête. => Donner un composant que l'on veut sur une entité par exemple.
pub fn npc_goal_reached(
    mut commands: Commands,
    npc_entity_goal_q: Query<(Entity, &Goal), (With<Npc>, With<CheckGoal>, With<Turn>, Without<IsDead>)>,    // Si pas de Turn, ca tournera en boucle.
    entity_killed_q: Query<&IsDead>,
) {
    let mut to_remove = Vec::new();
    for (npc_entity, npc_goal) in npc_entity_goal_q.iter() {
        match npc_goal.id {
            GoalType::KillEntity{id} => {
                if let Ok(_entity_dead) = entity_killed_q.get(id) {
                    println!("Goal {:?} for NPC {:?} is resolved.", npc_goal.id, npc_entity);
                    // Ici on retire le Planning car on a un seul goal. 
                    to_remove.push(npc_entity);
                    
                } else {
                    commands.entity(npc_entity).insert(Planning);
                    println!("Goal {:?} for NPC {:?} is still not true and need to be accomplished.", npc_goal.id, npc_entity);                    
                }
            },
            GoalType::None => {}
        };
    };
    for entity in to_remove {
        commands.entity(entity).remove::<CheckGoal>();
    }
}

/* 0.19h : Peut servir plus tard si on veut diviser un peu decision encore pluss. 


pub fn npc_planning_hit_melee_target(
    mut commands: Commands,
    npc_entity_plan_melee_q: Query<(Entity, &PlanHitMelee), (With<Npc>, With<Turn>, Without<IsDead>)>,
    position_q: Query<&BoardPosition>,
    fighters_q: Query<Entity, (With<Stats>, With<Health>, Without<IsDead>)>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
    npc_action_points: Query<&ActionPoints, (With<Npc>, With<Turn>, Without<IsDead>)>,
) {
    for (npc_entity, plan_melee) in npc_entity_plan_melee_q.iter() {
        commands.entity(npc_entity).remove::<PlanHitMelee>();

        let Ok(npc_position) = position_q.get(npc_entity) else { continue;};
        let Ok(target_position) = position_q.get(plan_melee.target) else { continue;};
        let Ok(_target_fighter) = fighters_q.get(plan_melee.target) else {continue;};
        let Ok(_npc_fighter) = fighters_q.get(npc_entity) else {continue;};

        // !!: Changer ça, ca n'a rien à faire ici? Sans ça, le NPC plannifie en boucle son mouvement. Cela doit être une condition pour pouvoir utiliser cette action.
        let Ok(npc_action_points) = npc_action_points.get(npc_entity) else { continue;};
        if !enought_ap(npc_action_points, plan_melee.ap_cost) {
            ev_endturn.send(EntityEndTurnEvent {entity : npc_entity}); 
            continue;
        };
        println!("NPC {:?} has enought AP to hit its target", npc_entity);

        //TO CHANGE: C'est degueulasse. Et un jour, le "in reach" pourra depasser 1.
        if (target_position.v.x - npc_position.v.x).abs() < 2 && (target_position.v.y - npc_position.v.y).abs() < 2 {
            println!("NPC {:?} at {:?} is in reach of {:?} and can hit its target.", npc_entity, npc_position, target_position);
            commands.entity(npc_entity).insert(WantToHit { mode: AttackType::MELEE, target: target_position.v }); 
        }      
    }
}


pub fn npc_planning_movement_to_destination(
    mut commands: Commands,
    npc_entity_planmove_q: Query<(Entity, &PlanMove), (With<Npc>, With<Turn>, Without<IsDead>)>,
    position_q: Query<&BoardPosition>,
    board: Res<Map>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
    npc_action_points: Query<&ActionPoints, (With<Npc>, With<Turn>, Without<IsDead>)>,
    
) {
    for (npc_entity, plan_move) in npc_entity_planmove_q.iter() {
        commands.entity(npc_entity).remove::<PlanMove>();
        let Ok(npc_position) = position_q.get(npc_entity) else { continue;};

        // TODO : Changer ça, ca n'a rien à faire ici? Sans ça, le NPC plannifie en boucle son mouvement. Cela doit être une condition pour pouvoir utiliser cette action.
        let Ok(npc_action_points) = npc_action_points.get(npc_entity) else { continue;};
        if npc_action_points.current < plan_move.ap_cost { //< AP_COST_MELEE { 
            ev_endturn.send(EntityEndTurnEvent {entity : npc_entity}); 
            println!("NPC {:?} n'a pas assez d'AP pour se deplacer.", npc_entity);
            continue;
        }

        let path_to_destination = find_path(
            npc_position.v,
            plan_move.destination, 
            &board.entity_tiles.keys().cloned().collect(), 
            &query_occupied.iter().map(|p| p.v).collect(),
            true,  // Obligé de l'avoir en true, sinon on considère que pas de route pour s'y rendre.
        );

        if path_to_destination.is_none() { 
            ev_endturn.send(EntityEndTurnEvent { entity: npc_entity}); 
            println!("NPC {:?} n'a pas de path vers la destination.", npc_entity);
            continue;
        }

        
        if let Some(path) = path_to_destination { 
            println!("NPC {:?} has a path.", npc_entity);
            // Le path peut être assez important sans filtrage de view ici. Normalement le systeme d'AP & WantToMove gère ca car depense de AP dedans une fois valide.
            //DEBUG:
            let pathlen = path.clone();
            println!("pathlen is {:?}", pathlen.len());
            commands.entity(npc_entity).insert(WantToMove { entity: npc_entity, path: path, target: Some(plan_move.destination)});
            println!("NPC {:?} receives a WantToMove with his path. Path len is : {:?}", npc_entity, pathlen.len());
        } else {
            println!("NPC {:?} has no path", npc_entity);
            // !! A voir comment on gère ça autrement. Normalement on devrait regarder les actions possibles et si aucune convient on passe le tour.
            // Pour le moment on a qu'une action possible donc pas de souci.
            ev_endturn.send(EntityEndTurnEvent { entity: npc_entity }); 
            continue;
        };
    }
}
*/