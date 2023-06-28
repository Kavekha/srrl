use bevy::prelude::*;

pub mod components;
mod events;

use crate::{states::{GameState, EngineState}, game::combat::components::{ActionPoints, CombatInfos}};

use self::{events::{CombatTurnQueue, CombatTurnStartEvent, CombatTurnNextEntityEvent, CombatTurnEndEvent, EntityEndTurnEvent, Turn}, components::CurrentEntityTurnQueue};

use super::{pieces::components::{Health, Stats, Npc}, player::{Player}, ui::ReloadUiEvent, rules::consume_actionpoints};



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
        //SECONDE REFACTO
            .add_state::<CombatState>()     // Les infos du combat: Tour, Personnage dont c'est le tour.
            
            .init_resource::<CombatTurnQueue>()     // Les personnages qui vont agir pendant ce tour.
            .init_resource::<CurrentEntityTurnQueue>()      // L'entité dont les actions vont être résolus pour ce tour.

            .add_event::<CombatTurnStartEvent>()        // Lance le tour.
            .add_event::<CombatTurnNextEntityEvent>()           // Envoyé pour prendre le nouvel acteur.
            .add_event::<CombatTurnEndEvent>()              // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.

            .add_event::<EntityEndTurnEvent>()                  // Envoyé par l'Entité qui mets volontairement fin à son tour.    //TODO : Meilleur nom: c'est une Action d'un NPC.                 
            // Init Combat.
            .add_systems(OnEnter(GameState::GameMap), combat_start)      // On lance le Combat dés l'arrivée en jeu. //TODO : Gestion de l'entrée / sortie en combat.
           // Le tour commence.
           .add_systems(Update, combat_turn_start.run_if(on_event::<CombatTurnStartEvent>()))
           // On prends l'entité dont c'est le tour. On passe en TurnUpdate
           .add_systems(Update, combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()))
            // toutes les entités ont fait leur tour.
            .add_systems(Update, combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()))

            // Generation des actions à faire.
            .add_systems(Update, combat_input.run_if(in_state(GameState::GameMap)))
            .add_systems(Update, plan_action_forfeit.run_if(in_state(GameState::GameMap)))
            
            // Gestion des actions demandées.
            .add_systems(Update, action_entity_end_turn.run_if(in_state(GameState::GameMap)))


            // Check de la situation PA-wise.
            .add_systems(Update, combat_turn_entity_check.run_if(in_state(GameState::GameMap)))

            // TODO: Quitter le combat. PLACEHOLDER.
            .add_systems(OnExit(GameState::GameMap), combat_end)
                
            ;
    }
}



/// Donne AP aux participants, créé le CombatInfos ressource, passe en StartTurn.
pub fn combat_start(    
    mut commands: Commands,
    mut combat_state: ResMut<NextState<CombatState>>,
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
    // On lui donne le composant "Turn".
    commands.entity(entity).insert(Turn);

    combat_state.set(CombatState::TurnUpdate);

    /* 
    // si joueur, on mets le PlayerTurn State.
    if let Ok(_player) = q_player.get(entity) {
        //println!("Next entity is player : {:?}", entity);
        combat_state.set(CombatState::PlayerTurn);
    } else {    
        //println!("Next entity is a npc : {:?}", entity);
        combat_state.set(CombatState::TurnUpdate);
    }
    */
}

pub fn combat_turn_end(    
    mut combat_state: ResMut<NextState<CombatState>>,
    mut ev_newturn: EventWriter<CombatTurnStartEvent>,
){
    println!("Combat turn End.");    
    ev_newturn.send(CombatTurnStartEvent);
    //combat_state.set(CombatState::StartTurn);
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

/// NPC : Generate / Choice to forfeit their turn.
pub fn plan_action_forfeit(
    combat_info: Res<CombatInfos>,
    mut query_npc: Query<(Entity, &ActionPoints, &Turn), With<Npc>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
){
    println!("Planning forfeit...");
    let Some(_entity) = combat_info.current_entity else { return };  //TODO : Toujours necessaire avec le Component Turn?
    for (entity, _action_points, _turn) in query_npc.iter() {
        //TODO : Dans quelles circonstances un NPC decide de Forfeit.
        println!("planning: Entity is a NPC.");
        ev_endturn.send(EntityEndTurnEvent {entity})     
    }  
}

/// Gestion de l'action de forfeit.
pub fn action_entity_end_turn(
    mut ev_endturn: EventReader<EntityEndTurnEvent>,
    mut query_character_turn: Query<(Entity, &mut ActionPoints, &Turn, Option<&Player>)>,
    mut combat_queue: ResMut<CurrentEntityTurnQueue>,
    //mut combat_state: ResMut<NextState<CombatState>>,
    //mut ev_interface: EventWriter<ReloadUiEvent>,    
    //mut ev_tick: EventWriter<CombatTickEvent>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    println!("action entity turn");
    for event in ev_endturn.iter() {
        //L'entité n'a pas de Action points / Pas son tour, on ignore.
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { continue };
        let (_entity, mut action_points, _turn, is_player) = entity_infos;

        let lost_value = action_points.max.saturating_add(0);
        consume_actionpoints(&mut action_points, lost_value);
        
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
        }
    }
}

/// Regarde si tous les PA ont été dépensé par le personnage dont c'est le tour.
/// Si c'est le cas, passe au perso suivant.
pub fn combat_turn_entity_check(
    mut commands: Commands,
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
                commands.entity(entity).remove::<Turn>();
                ev_next.send(CombatTurnNextEntityEvent);
           } else if is_player.is_some() {
                println!("This entity has AP and is the Player.");
                //combat_state.set(CombatState::PlayerTurn);
           } else {
           println!("This entity has AP but is not the player");
           }
        }
       // println!("Turn Entity check: {:?} turn.", entity);
    }    
}

/// Retire les ActionPoints, Remove CombatInfos, change State.
pub fn combat_end(
    mut commands: Commands,
    fighters: Query<(Entity, &ActionPoints)>,
    mut combat_state: ResMut<NextState<CombatState>>,
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

