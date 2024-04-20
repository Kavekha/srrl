// ==> DOCUMENTATION 0.19
/*
On recoit un event RefreshActionCostEvent.
On recalcule tout ce qui est utile pour prise de décision du joueur: Cout AP, chemin, cible, type d'attaque etc.

// 0.20n v2
Impl ActionInfos.

J'ai un personnage.

Pas mon tour : WAITING
Curseur est sur une Tuile :
    Tuile hidden => CANT_SEE
    Tuile known => MOVING
    Tuile visible => 
        Si Attack.Ranged => SHOOTING
        Si Attack.Melee =>
            Vide => MOVING 
            Enemi => PUNCHING
*/
use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{game::{ pieces::components::{Health, Occupier, Stats}, player::{Cursor, Player}, tileboard::components::BoardPosition, ui::events::ReloadUiEvent, visibility::components::View}, map_builders::map::Map, vectors::{find_path, Vector2Int}};

use super::{ combat_system::components::{ActionPoints, AttackType, IsDead}, components::CombatInfos, events::RefreshActionCostEvent, rules::{AP_COST_MELEE, AP_COST_MOVE, AP_COST_RANGED}};



// v0.20n
#[derive(Resource, Debug)]
pub struct ActionInfos {
    pub available_action: CharacterAction,
    pub target: Option<Vector2Int>,     // Il y a un fighter a cette position là (Position, Health, Stats, not isDead, not Player)   
    pub attack: Option<AttackType>,  // 0.19.c      
    pub cost: Option<u32>,          // Combien coutera l'action en AP.
    pub path: Option<VecDeque<Vector2Int>>, //Si accessible, on a quelque chose ici: le trajet pour se rendre à la destination (Non enregistrée)
    pub entity: Option<Entity>,     // C'est le joueur. // CAREFUL : Un jour on aura plus de un personnage.
    
}

#[derive(Debug)]
pub enum CharacterAction{
    NONE,
    WAITING,
    CANTSEE,
    MOVING,
    TARGETING,
    PUNCHING
}

// v2 : 0.20n : Plutot que diviser la gestion du curseur, on passe par ici finalement.
pub fn update_action_infos(
    query_occupied: Query<&BoardPosition, With<Occupier>>,     
    mut ev_interface: EventWriter<ReloadUiEvent>,
    mut ev_refresh_action: EventReader<RefreshActionCostEvent>,
    entity_player_q: Query<Entity,With<Player>>,
    //turn_q: Query<&Turn>,
    view_q: Query<&View, With<Player>>,
    piece_position: Query<&BoardPosition, (With<Health>, With<Stats>, Without<IsDead>, Without<Player>)>,
    board: Res<Map>,
    mut action_infos: ResMut<ActionInfos>,
    cursor: Res<Cursor>,
    position_q: Query<&BoardPosition>,
    action_points_q: Query<&ActionPoints>,
    combat_infos: Res<CombatInfos>,
) {
    for _event in ev_refresh_action.read() {
        //info!("Event RefreshActionCostEvent received.");
        //Reset:
        action_infos.cost = None;
        action_infos.path = None;
        action_infos.target = None; //Some(cursor.grid_position);
        action_infos.entity = None;

        // Determiner l'action disponible.
        action_infos.available_action = CharacterAction::NONE;
        let Ok(entity) = entity_player_q.get_single() else {
            //info!("ActionInfos: No player.");
            ev_interface.send(ReloadUiEvent);
            return };
        action_infos.entity = Some(entity);

        // On ne peut pas récuperer vraiment le Turn au bon moment sans cumuler les checks (CombatTurnCheck). Mieux vaut regarder le current_entity de CombatInfos.
        //let Ok(_is_turn) = turn_q.get(entity) else {         
        if combat_infos.current_entity != action_infos.entity {     // Si ce n'est pas le joueur....
            action_infos.available_action = CharacterAction::WAITING; 
            //info!("ActionInfos: Not player turn. Current is {:?} vs action is {:?}", combat_infos.current_entity, action_infos.entity);
            ev_interface.send(ReloadUiEvent);
            return 
        };
        //info!("ActionInfos: Player turn.");

        let Ok(view) = view_q.get(entity) else { return };
        let tile_position = cursor.grid_position;      

        // Tile jamais vue.
        if !board.is_revealed(tile_position.x, tile_position.y) {
            //info!("ActionInfos: tile not revealed");
            action_infos.available_action = CharacterAction::CANTSEE;
            ev_interface.send(ReloadUiEvent);
            return 
        } 
        if !view.visible_tiles.contains(&tile_position) {
            action_infos.available_action = CharacterAction::MOVING;
            ev_interface.send(ReloadUiEvent);
        } else {
            // Je vois la destination. Y a-t-il une cible?
            //info!("Action info: Je vois la destination. ");
            if piece_position.iter().any(|board_position| board_position.v == tile_position) {
                action_infos.target = Some(tile_position);
                match &action_infos.attack {
                    Some(AttackType::MELEE) => action_infos.available_action = CharacterAction::PUNCHING,
                    Some(AttackType::RANGED) => action_infos.available_action = CharacterAction::TARGETING,
                    None => action_infos.available_action = CharacterAction::MOVING,
                }
            } else {
                action_infos.available_action = CharacterAction::MOVING;
            }
            ev_interface.send(ReloadUiEvent);
        }

        // ---- A ce stade on ne peut avoir que : MOVING, PUNCHING, SHOOTING.
        //info!("Action info: Moving, Punching, Shooting is available.");

        // On calcule un trajet jusqu'à la cible. si Cible, on ne verifie pas si on peut marcher sur la dernière case (car on ne pourrait pas: elle est utilisée par la Target)
        let Ok(position) = position_q.get(entity) else {
             return };

        let path_to_destination = find_path(
            position.v,
            tile_position,
            &board.entity_tiles.keys().cloned().collect(),
            &query_occupied.iter().map(|p| p.v).collect(),
            action_infos.target.is_some(),
        ); 

        let Some(path) = path_to_destination else { 
                //DEBUG: println!("Pas de Path");
            return };
        
        // On remonte le cout en AP de l'action en cours.
       // info!("Action info: Calculate AP : attack is {:?}", action_infos.attack);
        let mut ap_cost: u32;
        match &action_infos.attack {
            None => {
                // Le chemin pour se rendre à la cible.
                ap_cost = path.len() as u32;
                // S'il y a une Target (un fighter), on ajoute alors le cout du Melee - le cout du deplacement (car on ne fera pas le dernier pas)
                if action_infos.target.is_some() {
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
                        if action_infos.target.is_some() {
                            let ap_melee_cost = AP_COST_MELEE.saturating_sub(AP_COST_MOVE); // REMEMBER : En melee, le dernier pas est sur la cible donc il faut le retirer.
                            ap_cost = ap_cost.saturating_add(ap_melee_cost)
                        }
                    },
                    AttackType::RANGED => {
                        // On attaque à distance ici, s'il y a une cible. Sinon on ne peut rien faire.
                        if action_infos.target.is_some() {
                            ap_cost = AP_COST_RANGED;
                        } else {
                            ev_interface.send(ReloadUiEvent);
                            return  // Cost sera a None, car on ne peut rien faire si pas de vraie cible.
                        }
                    }
                };
            },
        };
        //Si on a autant ou moins de AP disponibles que le cout mentionné, alors on affiche le cout AP et enregistre le Path.
        let Ok(action_points) = action_points_q.get(entity) else { return };
        if action_points.current >= ap_cost {
            action_infos.cost = Some(ap_cost);
            action_infos.path = Some(path);
        };
        ev_interface.send(ReloadUiEvent);
        //info!("Action Infos finished.");
    }
}



/* 
pub fn update_action_infos_v1(
    mut ev_refresh_action: EventReader<RefreshActionCostEvent>,
    query_character_turn: Query<(Entity, &ActionPoints), With<Player>>, 
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
    mut action_infos: ResMut<ActionInfos>,
    cursor: Res<Cursor>,
    piece_position: Query<&BoardPosition, (With<Health>, With<Stats>, Without<IsDead>)>,
    mut ev_interface: EventWriter<ReloadUiEvent>,
    view_q: Query<&View, With<Player>>,

) {
    for _event in ev_refresh_action.read() {
        //info!("Updating ActionInfos ");
        //Reset:
        action_infos.cost = None;
        action_infos.path = None;
        action_infos.target = None; //Some(cursor.grid_position);
        action_infos.entity = None;

        let Ok(player_infos) = query_character_turn.get_single() else { 
            //info!("action infos: No player info");
            return };
        let (entity, action_points) = player_infos;
        action_infos.entity = Some(entity);
        let Ok(position) = piece_position.get(entity) else { return };

        let tile_position = cursor.grid_position;
        if !board.entity_tiles.contains_key(&tile_position) { 
            //info!("Create action: out of map for {:?} with position: {:?}", entity, position);
            return }
        
        // En visu?
        let mut position_is_seen = true;
        if !board.is_revealed(tile_position.x, tile_position.y) {
            if let Ok(view) = view_q.get(entity) {
                if !view.visible_tiles.contains(&tile_position) {
                    return;
                }
            }
            position_is_seen = false;
        }        

        // Il y a un fighter ici (Fighter = Health, Stats & N'est pas Mort.)
        let mut has_target = false;
        if position_is_seen {
            if piece_position.iter().any(|board_position| board_position.v == tile_position) {
                has_target = true;
                action_infos.target = Some(tile_position);
            }
        }

        // 0.19e : Visuel 
        // 0.20n : Si dans la vue, alors attaquable. Pas de "in sight" pour le joueur. On verra plus tard si arme a du range.
        /* 
        if has_target {
            let Ok(_in_los) = is_in_sight(&board, &position.v, &action_infos.target.unwrap(), RANGED_ATTACK_RANGE_MAX) else {
                println!("Has target, not in view");
                continue;
            };
        }  
        */        

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
*/