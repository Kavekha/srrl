use bevy::{prelude::*, reflect::List};

pub mod components;
use crate::{states::GameState, game::combat::components::{ActionPoints, CombatTurnEndEvent}};

use self::components::{CombatTurnQueue, EntityEndTurnEvent, CombatTurnNextEntityEvent};

use super::{pieces::components::{Health, Stats}, player::Player, ui::ReloadUiEvent};



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
            .init_resource::<CombatTurnQueue>()

            .add_state::<CombatState>()

            .add_event::<EntityEndTurnEvent>()      // Envoyé par l'Entité qui mets volontairement fin à son tour.
            .add_event::<CombatTurnEndEvent>()              // Envoyé quand plus aucun acteur dans la Queue du Tour de Combat.
            .add_event::<CombatTurnNextEntityEvent>()           // Envoyé pour prendre le nouvel acteur.

            // Init Combat.
            .add_systems(OnEnter(GameState::GameMap), combat_start)
            .add_systems(OnExit(GameState::GameMap), combat_end)

            .add_systems(Update, combat_input.run_if(in_state(GameState::GameMap)))

            // Deroulement du tour.            
            // On ajoute les participants au combat.
            .add_systems(OnEnter(CombatState::StartTurn), combat_turn_start)
            .add_systems(Update, combat_turn_update.run_if(in_state(CombatState::TurnUpdate)))
            .add_systems(Update, combat_turn_next_entity.run_if(on_event::<CombatTurnNextEntityEvent>()))
            
            .add_systems(Update, entity_end_turn.run_if(on_event::<EntityEndTurnEvent>()))
            .add_systems(Update, combat_turn_end.run_if(on_event::<CombatTurnEndEvent>()))
            ;
    }
}


#[derive(Resource)]
pub struct CombatInfos {
    pub turn: u32,
    pub current_entity: Option<Entity>
}

/// Donne AP aux participants, créé le CombatInfos ressource, passe en PlayerTurn.
pub fn combat_start(    
    mut commands: Commands,
    mut combat_state: ResMut<NextState<CombatState>>,
    fighters: Query<(Entity, &Health, &Stats, Option<&Player>)>,
) {    
    // TODO: Adds this by default?
    for (fighter_id, fighter_health, fighter_stat, fighter_player) in fighters.iter() {
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
    npc_query: Query<Entity, (With<ActionPoints>, Without<Player>)>,
    player_query: Query<Entity, (With<ActionPoints>, With<Player>)>,
    mut action_query: Query<&mut ActionPoints>,
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_next: EventWriter<CombatTurnNextEntityEvent>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    for npc in npc_query.iter() {
        let Ok(mut action) = action_query.get_mut(npc) else {continue};
        action.current = action.current.saturating_add(10);
        queue.0.insert(0, npc);
    }
    /*
    queue.0.extend(
        npc_query.iter()
    );
    */
    for player in player_query.iter() {
        let Ok(mut action) = action_query.get_mut(player) else {continue};
        action.current = action.max;
        queue.0.insert(0, player);
        ev_interface.send(ReloadUiEvent);
    }

    /* 
    //We add player last, so they are the first to play.
    queue.0.extend(
        player_query.iter()
    );    
    */
    println!("Combat turn queue has {:?} messages.", queue.0.len());
    println!("Sending Next Entity");
    ev_next.send(CombatTurnNextEntityEvent);

}

/// Quand plus de PA, on passe à l'entité suivante.
/// Si plus d'entités dans le tour, on passe au tour suivant.
pub fn combat_turn_next_entity(
    mut queue: ResMut<CombatTurnQueue>,
    mut ev_turn_end: EventWriter<CombatTurnEndEvent>,
    q_player: Query<&Player>,
    mut current_combat: ResMut<CombatInfos>,    
    mut combat_state: ResMut<NextState<CombatState>>,
) {
    let Some(entity) = queue.0.pop_front() else {
        ev_turn_end.send(CombatTurnEndEvent);
        println!("--> Combat Turn End ! <---");
        return;
    };
    current_combat.current_entity = Some(entity);
    println!("Combat turn next entity: Entity is {:?}", entity);
    if let Ok(_player) = q_player.get(entity) {
        combat_state.set(CombatState::PlayerTurn);
    } else {
        combat_state.set(CombatState::TurnUpdate);
    }
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
            if ap_entity.current <= 0 {
                ev_next.send(CombatTurnNextEntityEvent);
           } else {
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

pub fn entity_end_turn(
    mut ev_endturn: EventReader<EntityEndTurnEvent>,
    mut action_points_q: Query<&mut ActionPoints>,
    mut ev_interface: EventWriter<ReloadUiEvent>,    
    mut combat_state: ResMut<NextState<CombatState>>,
) {
    for event in ev_endturn.iter() {
        if let Ok(mut action_points) =  action_points_q.get_mut(event.entity) {
            consume_actionpoints(&mut action_points, 100);   //TODO : Better way?
            //action_points.current = 0;
            ev_interface.send(ReloadUiEvent);
            println!("Turn End for {:?}. Action points : {:?}", event.entity, action_points.current);
        }        
    }    
    println!("Un petit turnupdate pour la forme!");
    combat_state.set(CombatState::TurnUpdate);
}

pub fn consume_actionpoints(
    actionpoints_component: &mut ActionPoints,
    lost_value: u32,
) {
    actionpoints_component.current = actionpoints_component.current.saturating_sub(lost_value);
}


pub fn combat_input(
    keys: Res<Input<KeyCode>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,  
    player_query: Query<(Entity, With<Player>)>,
){
    if keys.just_pressed(KeyCode::T) {
        if let Ok(result) = player_query.get_single() {
            let entity = result.0;
            ev_endturn.send(EntityEndTurnEvent {entity});
            println!("End of round for {:?}.", entity);
        }
        
    }
}