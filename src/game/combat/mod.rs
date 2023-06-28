use std::collections::VecDeque;

use bevy::{prelude::*, transform::commands};

pub mod components;
mod events;

use crate::{states::{GameState, EngineState}, game::combat::{components::{ActionPoints, CombatInfos, MovePath}, events::AnimateEvent}, map_builders::map::Map, vectors::{find_path, Vector2Int}, render::{components::PathAnimator, get_final_world_position}};

use self::{events::{CombatTurnQueue, CombatTurnStartEvent, CombatTurnNextEntityEvent, CombatTurnEndEvent, EntityEndTurnEvent, Turn, EntityMoveEvent, EntityTryMoveEvent}, components::CurrentEntityTurnQueue};

use super::{pieces::components::{Health, Stats, Npc, Occupier, Piece}, player::{Player, Cursor}, ui::ReloadUiEvent, rules::consume_actionpoints, tileboard::components::BoardPosition};




pub const AP_COST_MOVE:u32 = 1;



pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
        //SECONDE REFACTO
            .init_resource::<CombatTurnQueue>()     // Les personnages qui vont agir pendant ce tour.
            .init_resource::<CurrentEntityTurnQueue>()      // L'entité dont les actions vont être résolus pour ce tour.

            .add_event::<CombatTurnStartEvent>()        // Lance le tour.
            .add_event::<CombatTurnNextEntityEvent>()           // Envoyé pour prendre le nouvel acteur.
            .add_event::<CombatTurnEndEvent>()              // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.

            .add_event::<EntityEndTurnEvent>()                  // Envoyé par l'Entité qui mets volontairement fin à son tour.    //TODO : Meilleur nom: c'est une Action d'un NPC.                 
            .add_event::<EntityTryMoveEvent>()
            .add_event::<EntityMoveEvent>()
   
            .add_event::<AnimateEvent>()    //Animation //TODO : Deplacer.
            
            // Init Combat.
            .add_systems(OnEnter(GameState::GameMap), combat_start)      // On lance le Combat dés l'arrivée en jeu. //TODO : Gestion de l'entrée / sortie en combat.
           // Le tour commence.
           .add_systems(Update, combat_turn_start.run_if(on_event::<CombatTurnStartEvent>()))
           // On prends l'entité dont c'est le tour. On passe en TurnUpdate
           .add_systems(Update, combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()))
            // toutes les entités ont fait leur tour.
            .add_systems(Update, combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()).before(combat_turn_start))

            // Generation des actions à faire.
            .add_systems(Update, combat_input.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, plan_action_forfeit.run_if(in_state(GameState::GameMap)))
            
            // Check des actions demandées.
            .add_systems(Update, action_entity_try_move.run_if(in_state(GameState::GameMap)))
            
            // Gestion des actions demandées.
            .add_systems(Update, action_entity_end_turn.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, action_entity_move.run_if(in_state(GameState::GameMap)))

            // Check de la situation PA-wise.
            .add_systems(Update, combat_turn_entity_check.run_if(in_state(GameState::GameMap)))

            // ANIME : //TODO : Changer d'endroit.
            .add_systems(Update, walk_combat_animation.run_if(in_state(GameState::GameMap)))

            // TODO: Quitter le combat. PLACEHOLDER.
            .add_systems(OnExit(GameState::GameMap), combat_end)
                
            ;
    }
}



/// Donne AP aux participants, créé le CombatInfos ressource, passe en StartTurn.
pub fn combat_start(    
    mut commands: Commands,
    mut engine_state: ResMut<NextState<EngineState>>,   // TODO: Gerer le passage Combat / FreeMode.
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>)>,
) {    
    // TODO: Adds this by default?
    for (fighter_id, _fighter_health, _fighter_stat, _fighter_player) in fighters.iter() {
        commands.entity(fighter_id).insert(ActionPoints {max: 10, current: 0});
        println!("Action points added for {:?}", fighter_id);
    }
    commands.insert_resource(CombatInfos {turn: 0, current_entity: None});
    //combat_state.set(CombatState::StartTurn);
    ev_newturn.send(CombatTurnStartEvent);
    engine_state.set(EngineState::None);
    println!("Combat start!");
}


/// Ajoute les Participants du Turn au Combat dans la queue CombatTurnQueue.
pub fn combat_turn_start(
    mut action_query: Query<(Entity, &mut ActionPoints)>,
    npc_query: Query<Entity, (With<ActionPoints>, Without<Player>)>,
    player_query: Query<Entity, (With<ActionPoints>, With<Player>)>,
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    // On redonne les PA à tout le monde.
    println!("Combat turn start");
    for (_entity, mut action_points) in action_query.iter_mut() {
        action_points.current = action_points.max;
    }
    // On mets à jour l'interface pour les AP du joueur.
    ev_interface.send(ReloadUiEvent);

    // On mets les gens dans la CombatTurnQueue pour ce tour.
    // Npc d'abord
    queue.0.extend(
        npc_query.iter()
    );  
    // Player à la fin pour qu'il joue en premier.
    if let Ok(player) = player_query.get_single() {
        queue.0.insert(0, player);
    }
    println!("Combat turn queue has {:?} messages.", queue.0.len());

    // On lance le TurnNextEntity pour faire jouer le premier de la Queue.
    println!("Sending Next Entity");
    ev_next.send(CombatTurnNextEntityEvent);
}


/// On récupère le prochain combattant, puisque le précédent a fini.
pub fn combat_turn_next_entity(
    mut commands: Commands,
    mut queue: ResMut<CombatTurnQueue>,    
    mut ev_turn_end: EventWriter<CombatTurnEndEvent>,
    mut current_combat: ResMut<CombatInfos>,
) {
    let Some(entity) = queue.0.pop_front() else {
        // Plus de combattant: le tour est fini.
        println!("Combat Turn Next Entity: Plus de combattants dans la Queue.");        
        ev_turn_end.send(CombatTurnEndEvent);
        return;
    };
    // On mets à jour CombatInfos pour savoir qui est l'entité dont c'est le Tour.
    current_combat.current_entity = Some(entity);
    // On lui donne le composant "Turn".
    commands.entity(entity).insert(Turn);
}

pub fn combat_turn_end(    
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
    mut queue: ResMut<CombatTurnQueue>,
){
    println!("Combat turn End.");    
    queue.0.clear();
    ev_newturn.send(CombatTurnStartEvent);
}

/// Les events du Joueur.
pub fn combat_input(
    keys: Res<Input<KeyCode>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,  
    mut ev_try_move: EventWriter<EntityTryMoveEvent>,
    player_query: Query<(Entity, With<Player>)>,
    buttons: Res<Input<MouseButton>>,
    res_cursor: Res<Cursor>,    //TODO : On click event?
){
    if keys.just_pressed(KeyCode::T) {
        let Ok(result) = player_query.get_single() else { return };
        let entity = result.0;
        ev_endturn.send(EntityEndTurnEvent {entity});
        //println!("Player asked for End of round for {:?}.", entity);
    }
    if buttons.just_released(MouseButton::Left) {
        let Ok(result) = player_query.get_single() else { return };
        let entity = result.0;
        let destination = res_cursor.grid_position;
        ev_try_move.send(EntityTryMoveEvent {entity: entity, destination: destination});

    }
}

/// NPC : Generate / Choice to forfeit their turn.
pub fn plan_action_forfeit(
    combat_info: Res<CombatInfos>,
    mut query_npc: Query<(Entity, &ActionPoints, &Turn), With<Npc>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
){
    //println!("Planning forfeit...");
    let Some(_entity) = combat_info.current_entity else { return };  //TODO : Toujours necessaire avec le Component Turn?
    for (entity, _action_points, _turn) in query_npc.iter() {
        //TODO : Dans quelles circonstances un NPC decide de Forfeit.
        //println!("planning: Entity is a NPC.");
        ev_endturn.send(EntityEndTurnEvent {entity})     
    }  
}

/// Gestion de l'action de forfeit.
pub fn action_entity_end_turn(
    mut ev_endturn: EventReader<EntityEndTurnEvent>,
    mut query_character_turn: Query<(Entity, &mut ActionPoints, Option<&Player>), With<Turn>>,
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    //println!("action entity forfeit turn");
    for event in ev_endturn.iter() {
        //L'entité n'a pas de Action points / Pas son tour, on ignore.
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { continue };
        let (_entity, mut action_points, is_player) = entity_infos;

        let lost_value = action_points.max.saturating_add(0);
        consume_actionpoints(&mut action_points, lost_value);
        
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
        }
    }
}

/// Test de l'action Move.
pub fn action_entity_try_move(
    mut commands: Commands,
    mut query_character_turn: Query<(&ActionPoints, &BoardPosition, Option<&Player>), With<Turn>>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
    mut ev_try_move: EventReader<EntityTryMoveEvent>,
    mut ev_move: EventWriter<EntityMoveEvent>
){
    for event in ev_try_move.iter() {
        //println!("action entity try move");
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { return };
        let (action_points, position, _is_player) = entity_infos;

        let path_to_destination = find_path(
            position.v,
            event.destination,
            &board.entity_tiles.keys().cloned().collect(),
            &query_occupied.iter().map(|p| p.v).collect()
        ); 

        let Some(path) = path_to_destination else { return };
        let ap_cost = path.len() as u32;
        if action_points.current < ap_cost { return };

        let pathing = path.clone();


        println!("Try move: OK for {:?}. PA cost for moving is : {:?}", event.entity, ap_cost);
        commands.entity(event.entity).insert(MovePath {path: pathing});
        ev_move.send(EntityMoveEvent {entity: event.entity});


        /* 
        if !board.entity_tiles.contains_key(&event.destination) { return };    //Hors map.
        if board.is_blocked(event.destination.x, event.destination.y) { return };
        for occupier_position in query_occupied.iter() {
            if occupier_position.v == event.destination {
                return;
            }
        }
        */

    }
}


/// Gestion de l'action Move.
pub fn action_entity_move(
    mut commands: Commands,
    mut query_character_turn: Query<(Entity, &mut ActionPoints, &mut BoardPosition, &mut MovePath, Option<&Player>), With<Turn>>,
    mut ev_move: EventReader<EntityMoveEvent>,
    mut ev_interface: EventWriter<ReloadUiEvent>,
    mut ev_animate: EventWriter<AnimateEvent>
){    
    for event in ev_move.iter() {
        println!("action entity move");
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { continue };
        let (entity, mut action_points,mut board_position, mut move_path, is_player) = entity_infos;

        let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
        while !move_path.path.is_empty() {
            let destination = move_path.path.pop_front();
            let Some(new_position) = destination else { break };    // Normalement, il y a tjrs qq chose.
            board_position.v = new_position;
            action_points.current = action_points.current.saturating_sub(AP_COST_MOVE);
            path_animation.push_back(new_position);

            if is_player.is_some() {
                ev_interface.send(ReloadUiEvent);
            }
        }
        ev_animate.send(AnimateEvent {entity: entity, path: path_animation});
        // On supprime à la fin.
        commands.entity(entity).remove::<MovePath>();
        println!("Entity {:?} has MovePath removed.", entity);
        
        //TODO : anim
        //commands.entity(entity).insert(PathAnimator{path:VecDeque::from([target]), wait_anim: false});
    }
}

pub fn walk_combat_animation(    
    mut commands: Commands,
    mut ev_animate: EventReader<AnimateEvent>,
    query_piece: Query<&Piece>,
) {
    for ev in ev_animate.iter() {
        let Ok(piece) = query_piece.get(ev.entity) else { continue };
        let mut path = ev.path.clone();

        let mut path_animation: VecDeque<Vec3> = VecDeque::new();
        while !ev.path.is_empty() {
            let step = path.pop_front();
            let Some(current_step) = step else { break };
            let target = get_final_world_position(current_step, piece.size);
            path_animation.push_back(target);
        }
        commands.entity(ev.entity).insert(PathAnimator{path:VecDeque::from(path_animation), wait_anim: false});        
    }
}


/// Regarde si tous les PA ont été dépensé par le personnage dont c'est le tour.
/// Si c'est le cas, passe au perso suivant.
pub fn combat_turn_entity_check(
    mut commands: Commands,
    current_combat: ResMut<CombatInfos>,
    query_action_points: Query<(&ActionPoints, Option<&Player>)>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,    
) {
    //println!("Combat turn entity check...");
    // On recupere l'entité de CombatInfos.
    if let Some(entity) = current_combat.current_entity {
        //println!("There is a current entity in CombatInfos");
        if let Ok(entity_infos) = query_action_points.get(entity) {
            //println!("This entity has action points.");
            let (ap_entity, is_player) = entity_infos;
            //If no AP anymore, next entity turn.
            if ap_entity.current <= 0 {
                //println!("This entity has no AP: let's turn to next entity event.");
                commands.entity(entity).remove::<Turn>();
                ev_next.send(CombatTurnNextEntityEvent);
           } else if is_player.is_some() {
                //println!("This entity has AP and is the Player.");
                //combat_state.set(CombatState::PlayerTurn);
           } else {
            //println!("This entity has AP but is not the player");
           }
        }
       // println!("Turn Entity check: {:?} turn.", entity);
    }    
}

/// Retire les ActionPoints, Remove CombatInfos, change State.
pub fn combat_end(
    mut commands: Commands,
    fighters: Query<(Entity, &ActionPoints)>,
    mut queue: ResMut<CombatTurnQueue>,
){
    for (entity, _fighter) in fighters.iter() {
        commands.entity(entity).remove::<ActionPoints>();
    }
    commands.remove_resource::<CombatInfos>();
    //combat_state.set(CombatState::None);
    queue.0.clear();
    println!("Combat end!");
}

