use bevy::prelude::*;

use crate::{game::
    {combat::{
        components::{ActionPoints, AttackType, IsDead, WantToForfeit, WantToHit}, 
        events::Turn, 
        ia::components::PlanMove, 
        rules::{AP_COST_MELEE, AP_COST_RANGED, NPC_RANGED_ATTACK_RANGE_MAX}, 
    }, commons::is_in_sight, movements::components::WantToMove, pieces::components::{Melee, Npc, Occupier, Ranged, Walk}, tileboard::components::BoardPosition 
    },
    map_builders::map::Map, 
    vectors::find_path
    };

use super::components::{Goal, GoalType, Planning};




// IA verifie si elle est physiquement à coté d'un personnage, 
// Puis on regarde si on veut faire une action selon notre goal: taper pour le moment.
pub fn npc_ia_plan_when_adjacent(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, &ActionPoints, &Goal), (With<Npc>, With<Turn>, With<Planning>, With<Melee>, Without<IsDead>)>,
    position_q: Query<&BoardPosition>,    
) {
    let mut to_remove = Vec::new();
    for (npc_entity, npc_position, npc_ap, npc_goal) in npc_entity_fighter_q.iter() {
        // A moyen terme, faudra changer ce fonctionnement de regarder objectif par objectif, car l'info peut être utile pour plein de raison.
        match npc_goal.id {
            GoalType::KillEntity{id} => {
                let mut is_melee_position = false;
                let Ok(target_position) = position_q.get(id) else { continue; };
                if (target_position.v.x - npc_position.v.x).abs() < 2 && (target_position.v.y - npc_position.v.y).abs() < 2 {
                    //println!("NPC {:?} est coté de sa cible.", npc_entity);
                    is_melee_position = true;
                } 
                // Ici on check si on veut taper ou non.
                if is_melee_position {
                    if npc_ap.current >= AP_COST_MELEE {
                        commands.entity(npc_entity).insert(WantToHit { mode: AttackType::MELEE, target: target_position.v });
                        //println!("NPC {:?} is at position {:?} and their target is at {:?}. AP are OK so they wan't to HIT in MELEE.", npc_entity, npc_position.v, target_position.v);
                        to_remove.push(npc_entity); // On retire puisque le choix est OK.
                    } else {
                        //println!("NPC {:?} n'a pas les AP pour attaquer sa cible.", npc_entity);
                    }
                }
            },
            _ => {}
        };
    }
    // On retire le Planning à toutes les entités. // REMINDER : Hors de la boucle pour eviter les erreurs. TODO ailleurs.
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>();   
    }
}


pub fn npc_ia_plan_when_in_range(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, &ActionPoints, &Goal), (With<Npc>, With<Turn>, With<Planning>, With<Ranged>, Without<IsDead>)>,
    position_q: Query<&BoardPosition>,  
    board: Res<Map>,  
){
    let mut to_remove = Vec::new();
    for (npc_entity, npc_position, npc_ap, npc_goal) in npc_entity_fighter_q.iter() {
        //println!("NPC {:?} reflechit s'il doit ou peut utiliser une attaque Ranged.", npc_entity);
        match npc_goal.id {
            GoalType::KillEntity { id } => {
                if npc_ap.current < AP_COST_RANGED {  //AP_COST_MOVE {
                    //println!("NPC {:?} n'a pas les AP tirer.", npc_entity);
                    continue;
                };

                // In view?
                let Ok(target_position) = position_q.get(id) else { 
                    //println!("No position found for player. NPC can't check for target.");
                    continue;
                }; 
                if let Ok(_in_los) = is_in_sight(&board, &npc_position.v, &target_position.v, NPC_RANGED_ATTACK_RANGE_MAX) {
                    //println!("NPC {:?} peut tirer sur sa victime {:?}.", npc_entity, id);
                    commands.entity(npc_entity).insert(WantToHit { mode: AttackType::RANGED, target: target_position.v });
                    to_remove.push(npc_entity);
                };
                //println!("NPC {:?} a fini de reflechir à Ranged Attack et passe à autre chose.", npc_entity);
            },
            GoalType::None => {},
        };
    };
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>(); 
    }
}


// IA regarde autour d'elle et prends une decision a partir de ce qu'elle voit ou ne voit pas.
pub fn npc_ia_plan_on_view(
    mut commands: Commands,
    //npc_entity_fighter_q: Query<(Entity, &BoardPosition, &ActionPoints, &Goal), (With<Npc>, With<Turn>, With<Planning>, Without<IsDead>)>,
    npc_entity_fighter_q: Query<(Entity, &ActionPoints, &Goal), (With<Npc>, With<Turn>, With<Planning>, Without<IsDead>)>,
    position_q: Query<&BoardPosition>,     
    //board: Res<Map>,
) {    
    // Pas besoin de remove aussi: On pose un PlanSomething si c'est OK. Le PlanSomething sera géré à l'etape suivante.     // TODO : Ca reste assez vulnerable lors d'ajout ou changement.
    //for (npc_entity, _npc_position, npc_ap, npc_goal) in npc_entity_fighter_q.iter() {
    for (npc_entity, npc_ap, npc_goal) in npc_entity_fighter_q.iter() {
        match npc_goal.id {
            GoalType::KillEntity{id} => {   
                // Pas les AP.
                // !! WARNING: On est obligé de mettre du AP COST MELEE pour le moment CAR:
                //  1. J'ai 2 AP. J'ai le droit de me deplacer.
                //  2. La case où je veux me deplacer est celle de ma cible. Aller sur cette case pour le taper coute 3 PA.
                //  3. => Je n'ai pas 3 PA, je ne peux pas taper mais j'ai 1-2 PA, je peux bouger mais je ne peux pas bouger ou je veux car je n'ai pas 3 PA etc.
                // => TOFIX : 
                //      - Séparer Move / Taper.
                //      - Avoir un retour dans les WantTo pour sortir le NPC en cas de galere?
                //      - Pouvoir avoir le pathfinding sans aller sur la dernière case. Remove de la derniere etape à chaque fois?
                if npc_ap.current < AP_COST_MELEE {  //AP_COST_MOVE {
                    //println!("NPC {:?} n'a pas les AP pour se deplacer.", npc_entity);
                    continue;
                };

                let Ok(target_position) = position_q.get(id) else { 
                    //println!("No position found for player. NPC can't check for target.");
                    continue;
                }; 

                /* TODO : On fera ca en 0.20 
                println!("Npc {:?} a position {:?} verifie sa ligne de vue vers {:?}.", npc_entity, npc_position.v, target_position.v);
                let Ok(_in_los) = is_in_sight(&board, &npc_position.v, &target_position.v, NPC_VISION_RANGE_MAX) else {
                    println!("NPC {:?}: target {:?} is not in view.", npc_entity, id);
                    // TODO : Search for target.
                    continue;
                };
                println!("NPC {:?}: saw their target {:?}!", npc_entity, id);
                */
                
                // TODO : Ici on ne retire pas le planning par facilité. 
                // => PlanMove fait aller dans npc_ia_plan_approaching, qui verifie aussi le planning et le retirera si necessaire.
                // => En gros on voit : On va approaching, on voit pas, on ignore approaching et on va a la suite (forfeit)
                commands.entity(npc_entity).insert(PlanMove { destination: target_position.v});  
            }
            GoalType::None => {}
        };
    }
}

// IA veut approcher physiquement de la cible / tuile.
// Une partie des verifications sont faites dans npc_ia_plan_on_view: ia_plan_approaching est une sorte de sous-etape.
pub fn npc_ia_plan_approaching( 
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, &PlanMove), (With<Npc>, With<Turn>, Without<IsDead>, With<Walk>)>,   
    board: Res<Map>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
) {
    let mut to_remove_planning = Vec::new();
    let mut to_remove_plan_move = Vec::new();
    for (npc_entity, npc_position, npc_plan) in npc_entity_fighter_q.iter() {
        // Pas de Goal, on a déjà determiné cela avant.

        let path_to_destination = find_path(
            npc_position.v,
            npc_plan.destination, 
            &board.entity_tiles.keys().cloned().collect(), 
            &query_occupied.iter().map(|p| p.v).collect(),
            true,  // Obligé de l'avoir en true, sinon on considère que pas de route pour s'y rendre.
        );
        
        if let Some(path) = path_to_destination {
            //println!("NPC {:?} J'ai planifié un chemin pour moi.", npc_entity);
            commands.entity(npc_entity).insert(WantToMove { entity: npc_entity, path: path, target: Some(npc_plan.destination)});    
            to_remove_planning.push(npc_entity);
        } else {
            //println!("Pas de chemin pour moi.");
        }
        // Retrait du PlanMove sinon on ne refait plus le check View.   // REMINDER: C'etait très cool !
        to_remove_plan_move.push(npc_entity);
    }
    for entity in to_remove_planning {
        commands.entity(entity).remove::<Planning>();   
    }
    for entity in to_remove_plan_move {
        commands.entity(entity).remove::<PlanMove>();   
    }
}


pub fn npc_ai_plan_forfeit(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &ActionPoints, &Goal), (With<Npc>, With<Turn>, With<Planning>, Without<IsDead>)>,
) {
    let mut to_remove = Vec::new();
    for (npc_entity, _, _) in npc_entity_fighter_q.iter() {
        to_remove.push(npc_entity);
        //println!("NPC {:?} n'a rien a faire.", npc_entity);
        commands.entity(npc_entity).insert(WantToForfeit);
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>();
    }
}
