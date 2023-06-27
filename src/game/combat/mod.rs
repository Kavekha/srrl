use std::collections::VecDeque;

use bevy::prelude::*;

pub mod components;
pub mod combat_queue_system;
mod models;

use crate::{states::GameState, game::combat::{components::{ActionPoints, CombatTurnEndEvent, CombatInfos}, models::EndTurnAction}};
use self::{components::{CombatTurnQueue, EntityEndTurnEvent, CombatTurnNextEntityEvent, CombatTickEvent, CurrentEntityTurnQueue, InvalidPlayerCombatActionEvent, PlayerCombatActionEvent}, combat_queue_system::process_action_queue};
use super::{pieces::components::{Health, Stats, Actor, Npc}, player::{Player, PlayerActionEvent}, ui::ReloadUiEvent, actions::Action};



#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CombatState {
    #[default]
    None, 
    StartTurn,
    PlayerTurn,
    TurnUpdate
}


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<CombatState>()
            
            .init_resource::<CombatTurnQueue>()     // Les personnages qui vont agir pendant ce tour.
            .init_resource::<CurrentEntityTurnQueue>()      // L'entité dont les actions vont être résolus pour ce tour.

            .add_event::<CombatTurnNextEntityEvent>()           // Envoyé pour prendre le nouvel acteur.
            .add_event::<EntityEndTurnEvent>()                  // Envoyé par l'Entité qui mets volontairement fin à son tour.
            .add_event::<CombatTurnEndEvent>()              // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.
            .add_event::<InvalidPlayerCombatActionEvent>()
            .add_event::<PlayerCombatActionEvent>()
            .add_event::<CombatTickEvent>()

            // Init Combat.
            .add_systems(OnEnter(GameState::GameMap), combat_start)      // On lance le Combat dés l'arrivée en jeu. //TODO : Gestion de l'entrée / sortie en combat.
            // Le combat commence.
            .add_systems(OnEnter(CombatState::StartTurn), combat_turn_start)
            //Next entity turn.
            .add_systems(Update, combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()))
            

            // On attends les input du joueur. Cela doit generer un Event "J'ai fais une Action a prendre en compte"
            .add_systems(Update, combat_input.run_if(in_state(GameState::GameMap)))
            // Actions via Events.
            .add_systems(Update, action_entity_end_turn.run_if(on_event::<EntityEndTurnEvent>()))

            // Process des actions de la Queue CurrentEntityTurn.
            //.add_systems(Update, process_action_queue.run_if(in_state(CombatState::TurnUpdate)))
            .add_systems(Update, plan_action_forfeit.run_if(on_event::<CombatTickEvent>()).before(process_action_queue))
            .add_systems(Update, process_action_queue.run_if(on_event::<CombatTickEvent>()))            
            .add_systems(Update, combat_turn_entity_check.run_if(on_event::<CombatTickEvent>()).after(process_action_queue))
            

            
         
            // Gestion retour action PJ.
            //.add_systems(Update, invalid_player_action.run_if(on_event::<InvalidPlayerCombatActionEvent>()))
            //.add_systems(Update, valid_player_action.run_if(on_event::<PlayerCombatActionEvent>()))

            // toutes les entités ont fait leur tour.
            .add_systems(Update, combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()))

            // TODO: Quitter le combat. PLACEHOLDER.
            .add_systems(OnExit(GameState::GameMap), combat_end)





            //REFACTO
            /* 

            .add_event::<CombatTickEvent>()
            
            .add_systems(Update, combat_input_received.run_if(on_event::<PlayerActionEvent>()))
 */    
            // Deroulement du tour.            
            // On ajoute les participants au combat.
            //.add_systems(Update, combat_tick.run_if(in_state(CombatState::TurnUpdate)).before(combat_turn_update))
            //.add_systems(Update, combat_turn_update.run_if(in_state(CombatState::TurnUpdate)))
            
            
            

            
            ;
    }
}



/// Donne AP aux participants, créé le CombatInfos ressource, passe en StartTurn.
pub fn combat_start(    
    mut commands: Commands,
    mut combat_state: ResMut<NextState<CombatState>>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>)>,
) {    
    // TODO: Adds this by default?
    for (fighter_id, _fighter_health, _fighter_stat, _fighter_player) in fighters.iter() {
        commands.entity(fighter_id).insert(ActionPoints {max: 10, current: 10});
        println!("Action points added for {:?}", fighter_id);
    }
    commands.insert_resource(CombatInfos {turn: 0, current_entity: None});
    combat_state.set(CombatState::StartTurn);
    println!("Combat start!");
}


/// Ajoute les Participants du Turn au Combat dans la queue CombatTurnQueue.
/// TODO: Check if entity toujours vivante. !!
/// TODO : Meilleure gestion du regain AP.
pub fn combat_turn_start(
    mut action_query: Query<(Entity, &mut ActionPoints)>,
    npc_query: Query<Entity, (With<ActionPoints>, Without<Player>)>,
    player_query: Query<Entity, (With<ActionPoints>, With<Player>)>,
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    // On redonne les PA à tout le monde.
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
    mut queue: ResMut<CombatTurnQueue>,    
    mut ev_turn_end: EventWriter<CombatTurnEndEvent>,
    mut current_combat: ResMut<CombatInfos>,       
    q_player: Query<&Player>,
    mut combat_state: ResMut<NextState<CombatState>>,
) {
    let Some(entity) = queue.0.pop_front() else {
        // Plus de combattant: le tour est fini.
        //println!("Combat Turn Next Entity: Plus de combattants dans la Queue.");        
        ev_turn_end.send(CombatTurnEndEvent);
        return;
    };
    // On mets à jour CombatInfos pour savoir qui est l'entité dont c'est le Tour.
    current_combat.current_entity = Some(entity);

    // si joueur, on mets le PlayerTurn State.
    if let Ok(_player) = q_player.get(entity) {
        //println!("Next entity is player : {:?}", entity);
        combat_state.set(CombatState::PlayerTurn);
    } else {    
        //println!("Next entity is a npc : {:?}", entity);
        combat_state.set(CombatState::TurnUpdate);
    }
}

/// Les events du Joueur.
pub fn combat_input(
    keys: Res<Input<KeyCode>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,  
    player_query: Query<(Entity, With<Player>)>,
){
    if keys.just_pressed(KeyCode::T) {
        if let Ok(result) = player_query.get_single() {
            let entity = result.0;
            ev_endturn.send(EntityEndTurnEvent {entity});
            println!("Player asked for End of round for {:?}.", entity);
        }
        
    }
}

/// Une action accomplie par un personnage, déclenchée par un Event.
pub fn action_entity_end_turn(
    mut ev_endturn: EventReader<EntityEndTurnEvent>,
    mut actor_q: Query<&mut Actor>,    
    mut combat_queue: ResMut<CurrentEntityTurnQueue>,
    //mut combat_state: ResMut<NextState<CombatState>>,
    //mut ev_interface: EventWriter<ReloadUiEvent>,    
    mut ev_tick: EventWriter<CombatTickEvent>
) {
    for event in ev_endturn.iter() {
        let Ok(mut actor) = actor_q.get_mut(event.entity) else { continue;};

        let action = EndTurnAction(event.entity);
        actor.0 = vec![(Box::new(action), 0)];      // 0 => Player doesn't care for Action Score.
        combat_queue.0 = VecDeque::from([event.entity]);
    }
    //combat_state.set(CombatState::TurnUpdate);  //TODO : On va pas faire ça à chaque action...
    ev_tick.send(CombatTickEvent);
}


// ----- PROCESS ACTION AVEC DEPENSE DE PA ----- //


/// Regarde si tous les PA ont été dépensé par le personnage dont c'est le tour.
/// Si c'est le cas, passe au perso suivant.
pub fn combat_turn_entity_check(
    current_combat: ResMut<CombatInfos>,
    query_action_points: Query<(&ActionPoints, Option<&Player>)>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>, 
    mut combat_state: ResMut<NextState<CombatState>>,    
) {
    println!("Combat turn entity check...");
    // On recupere l'entité de CombatInfos.
    if let Some(entity) = current_combat.current_entity {
        //println!("There is a current entity in CombatInfos");
        if let Ok(entity_infos) = query_action_points.get(entity) {
            //println!("This entity has action points.");
            let (ap_entity, is_player) = entity_infos;
            //If no AP anymore, next entity turn.
            if ap_entity.current <= 0 {
                //println!("This entity has no AP: let's turn to next entity event.");
                ev_next.send(CombatTurnNextEntityEvent);
           } else if is_player.is_some() {
                //println!("This entity has AP and is the Player: let's go in PlayerTurnState.");
            combat_state.set(CombatState::PlayerTurn);
           }
           //println!("This entity has AP but is not the player");
        }
       // println!("Turn Entity check: {:?} turn.", entity);
    }    
}

pub fn combat_turn_end(    
    mut combat_state: ResMut<NextState<CombatState>>,
){
    println!("Combat turn End");
    combat_state.set(CombatState::StartTurn);
}

/// Retire les ActionPoints, Remove CombatInfos, change State.
pub fn combat_end(
    mut commands: Commands,
    fighters: Query<(Entity, &ActionPoints)>,
    mut combat_state: ResMut<NextState<CombatState>>,
){
    for (entity, _fighter) in fighters.iter() {
        commands.entity(entity).remove::<ActionPoints>();
    }
    commands.remove_resource::<CombatInfos>();
    combat_state.set(CombatState::None);
    println!("Combat end!");
}

/// NPC : Choice to forfeit their turn.
pub fn plan_action_forfeit(
    combat_info: Res<CombatInfos>,
    mut query_npc: Query<Entity, With<Npc>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
){
    println!("Planning forfeit...");
    let Some(entity) = combat_info.current_entity else { return };
    let Ok(_is_npc) = query_npc.get(entity) else { return };
    println!("planning: Entity is a NPC.");
    ev_endturn.send(EntityEndTurnEvent {entity})       
}


/* 

pub fn combat_input_received(
    mut combat_state: ResMut<NextState<CombatState>>,
){
    combat_state.set(CombatState::TurnUpdate);
}




pub fn combat_tick(
    mut ev_tick: EventWriter<CombatTickEvent>
) {
    ev_tick.send(CombatTickEvent);
}

pub fn combat_turn_update(
    mut commands: Commands,
    mut current_combat: ResMut<CombatInfos>,
    query_action_points: Query<&ActionPoints>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,
    query_player: Query<&Player>,   //DEBUG
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,      //DEBUG    
    
) {
    println!("Turn update begins...");
    // On recupere l'entité de CombatInfos.
    if let Some(mut entity) = current_combat.current_entity {
        if let Ok(ap_entity) = query_action_points.get(entity) {
            //If no AP anymore, next entity turn.
            if ap_entity.current <= 0 {
                ev_next.send(CombatTurnNextEntityEvent);
           }
        }
    }
    
    /* else {            
                //println!("Current AP for {:?}: {:?}", entity, ap_entity.current);
                //DEBUG
                if let Ok(_is_player) = query_player.get(entity) { 
                    println!("Player still have AP");
                } else {
                    ev_endturn.send(EntityEndTurnEvent {entity});
                }
           };
        } else {
            current_combat.current_entity = None;
            ev_next.send(CombatTurnNextEntityEvent);
        };
    } else {
        ev_next.send(CombatTurnNextEntityEvent);
    };  
    // On regarde s'il a des PA.
    // S'il n'en a plus, on le retire de CombatInfos et on passe à Next Entity
     */
}



*/







