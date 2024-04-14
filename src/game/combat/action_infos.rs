// ==> DOCUMENTATION 0.19
/*
On recoit un event RefreshActionCostEvent.
On recalcule tout ce qui est utile pour prise de décision du joueur: Cout AP, chemin, cible, type d'attaque etc.
*/
use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{game::{ commons::is_in_sight, pieces::components::{Health, Occupier, Stats}, player::{Cursor, Player}, tileboard::components::BoardPosition, ui::events::ReloadUiEvent}, map_builders::map::Map, vectors::{find_path, Vector2Int}};

use super::{ combat_system::components::{ActionPoints, AttackType, IsDead}, events::RefreshActionCostEvent, rules::{AP_COST_MELEE, AP_COST_MOVE, AP_COST_RANGED, RANGED_ATTACK_RANGE_MAX}};



#[derive(Resource)]
pub struct ActionInfos {
    pub cost: Option<u32>,          // Combien coutera l'action en AP.
    pub path: Option<VecDeque<Vector2Int>>, //Si accessible, on a quelque chose ici: le trajet pour se rendre à la destination (Non enregistrée)
    pub target: Option<Vector2Int>,     // Il y a un fighter a cette position là (Position, Health, Stats, not isDead)
    pub entity: Option<Entity>,     // C'est le joueur. // CAREFUL : Un jour on aura plus de un personnage.
    pub attack: Option<AttackType>  // 0.19.c      
}




pub fn update_action_infos(
    mut ev_refresh_action: EventReader<RefreshActionCostEvent>,
    query_character_turn: Query<(Entity, &ActionPoints), With<Player>>, 
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
    mut action_infos: ResMut<ActionInfos>,
    cursor: Res<Cursor>,
    piece_position: Query<&BoardPosition, (With<Health>, With<Stats>, Without<IsDead>)>,
    mut ev_interface: EventWriter<ReloadUiEvent>,

) {
    for _event in ev_refresh_action.read() {
        //println!("Updating ActionInfos ");
        //Reset:
        action_infos.cost = None;
        action_infos.path = None;
        action_infos.target = None; //Some(cursor.grid_position);
        action_infos.entity = None;

        let Ok(player_infos) = query_character_turn.get_single() else { 
            //println!("action infos: No player info");
            return };
        let (entity, action_points) = player_infos;
        action_infos.entity = Some(entity);
        let Ok(position) = piece_position.get(entity) else { return };

        let tile_position = cursor.grid_position;
        if !board.entity_tiles.contains_key(&tile_position) { 
            //println!("Create action: out of map for {:?} with position: {:?}", entity, position);
            return }

        // Il y a un fighter ici (Fighter = Health, Stats & N'est pas Mort.)
        let mut has_target = false;
        if piece_position.iter().any(|board_position| board_position.v == tile_position) {
            has_target = true;
            action_infos.target = Some(tile_position);
        }

        // 0.19e : Visuel 
        if has_target {
            let Ok(_in_los) = is_in_sight(&board, &position.v, &action_infos.target.unwrap(), RANGED_ATTACK_RANGE_MAX) else {
                println!("Has target, not in view");
                continue;
            };
        }          

        // On calcule un trajet jusqu'à la cible. si Cible, on ne verifie pas si on peut marcher sur la dernière case (car on ne pourrait pas: elle est utilisée par la Target)
        let path_to_destination = find_path(
            position.v,
            tile_position,
            &board.entity_tiles.keys().cloned().collect(),
            &query_occupied.iter().map(|p| p.v).collect(),
            has_target,
        ); 

        let Some(path) = path_to_destination else { 
                //DEBUG: println!("Pas de Path");
            return };

        // On remonte le cout en AP de l'action en cours.
        let mut ap_cost: u32;

        match &action_infos.attack {
            None => {
                // Le chemin pour se rendre à la cible.
                ap_cost = path.len() as u32;
                // S'il y a une Target (un fighter), on ajoute alors le cout du Melee - le cout du deplacement (car on ne fera pas le dernier pas)
                if has_target {
                    let ap_melee_cost = AP_COST_MELEE.saturating_sub(AP_COST_MOVE); // REMEMBER : En melee, le dernier pas est sur la cible donc il faut le retirer.
                    ap_cost = ap_cost.saturating_add(ap_melee_cost)
                }
            },
            Some(attack_type) => {
                match attack_type {
                    AttackType::MELEE => {
                        // Le chemin pour se rendre à la cible. // Doublon de none car on ne distingue pas le fait de se deplacer sur qq un ou sur une tuile.
                        ap_cost = path.len() as u32;
                        // S'il y a une Target (un fighter), on ajoute alors le cout du Melee - le cout du deplacement (car on ne fera pas le dernier pas)
                        if has_target {
                            let ap_melee_cost = AP_COST_MELEE.saturating_sub(AP_COST_MOVE); // REMEMBER : En melee, le dernier pas est sur la cible donc il faut le retirer.
                            ap_cost = ap_cost.saturating_add(ap_melee_cost)
                        }
                    },
                    AttackType::RANGED => {
                        // On attaque à distance ici, s'il y a une cible. Sinon on ne peut rien faire.
                        if has_target {
                            ap_cost = AP_COST_RANGED;
                        } else {
                            return  // Cost sera a None, car on ne peut rien faire si pas de vraie cible.
                        }
                    }
                };
            },
            //_ => println!("Not combat_input.")
        };

        //Si on a autant ou moins de AP disponibles que le cout mentionné, alors on affiche le cout AP et enregistre le Path.
        if action_points.current >= ap_cost {
            action_infos.cost = Some(ap_cost);
            action_infos.path = Some(path);
        };
        ev_interface.send(ReloadUiEvent);
        // DEBUG: println!("Update action finale: cost: {:?}, path: {:?}, target: {:?}, entity: {:?}", action_infos.cost, action_infos.path, action_infos.target, action_infos.entity);
    }
}
