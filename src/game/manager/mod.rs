use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{states::{GameState, EngineState, TurnSet}, render::GraphicsWaitEvent};

use super::{player::{PlayerActionEvent, Player}, actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent, PlayerActions, ActorQueue, CancelPlayerPendingActionsEvent}, pieces::components::Actor};


pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(GameState::GameMap), game_start) 
            //.add_systems(OnExit(GameState::GameMap), game_end)  

            .configure_set(Update, TurnSet::Logic.run_if(in_state(EngineState::TurnUpdate)).before(TurnSet::Animation))   
            .configure_set(Update, TurnSet::Animation.run_if(in_state(EngineState::TurnUpdate)).before(TurnSet::Tick))   
            .configure_set(Update, TurnSet::Tick.run_if(in_state(EngineState::TurnUpdate)))   

            .add_systems(OnEnter(EngineState::PlayerInput), turn_player_pending_actions)
            .add_systems(Update, turn_update_start.run_if(on_event::<PlayerActionEvent>()))
            .add_systems(Update, clear_player_pending_actions.run_if(on_event::<CancelPlayerPendingActionsEvent>()).before(turn_update_start))

            .add_systems(Update, turn_update_end.run_if(on_event::<ActionsCompleteEvent>()).before(turn_update_start))
            .add_systems(Update, turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()).before(turn_update_start))
            .add_systems(Update, tick.run_if(in_state(EngineState::TurnUpdate)).in_set(TurnSet::Tick))
            ;
    }
}

fn clear_player_pending_actions(
    mut player_queue: ResMut<PlayerActions>,  
) {
    player_queue.0.clear();
    //println!("Event! Clear Player Pending action!");
}

fn turn_player_pending_actions(
    mut player_queue: ResMut<PlayerActions>,  
    mut queue: ResMut<ActorQueue>,
    mut ev_action: EventWriter<PlayerActionEvent>,
    mut query_player: Query<(&mut Actor, With<Player>)>,
){
    if !player_queue.0.is_empty() {
        let mut player_actions = player_queue.0.drain(0..1).collect::<Vec<_>>();
        //println!("turn_player: an action is waiting : {:?}", player_actions.len());

        if let Some(last_action) = player_actions.pop() {
            let (action, entity) = last_action;
            if let Ok(mut actor) = query_player.get_component_mut::<Actor>(entity) {
                println!("turn player: action push.");
                actor.0.push((action, 0));
                queue.0 = VecDeque::from([entity]);                
                ev_action.send(PlayerActionEvent);
            } else {
                println!("No actor.");
            };
        } else {
            println!("turn player: No last action.");
        }     
    } else {
        println!("turn_player : player queue empty");
    }
}

fn game_start(
    mut next_state: ResMut<NextState<EngineState>>,
) {
    next_state.set(EngineState::PlayerInput);
    //println!("game_start: Engine set to PlayerInput");
}

fn game_end(
    mut next_state: ResMut<NextState<EngineState>>
) {
    next_state.set(EngineState::None);
    //println!("game_end: Engine set to None.")
}

fn turn_update_start(
    queue: ResMut<ActorQueue>,
    mut next_state: ResMut<NextState<EngineState>>,
    mut ev_tick: EventWriter<TickEvent>,    
) {
    println!("turn_update start: ActorQueue is {:?}", queue.0.len());
    next_state.set(EngineState::TurnUpdate);
    ev_tick.send(TickEvent);
    //println!("turn_update_start by PlayerActionEvent! Let's Send Tick and see if there is anything.")
}

fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    if ev_wait.iter().len() == 0 {
        ev_tick.send(TickEvent);
        //println!("tick!");
    }
    //println!("tick: ev_wait in process... {:?} to go.", ev_wait.iter().len());
}

fn turn_update_end(
    mut next_state: ResMut<NextState<EngineState>>
) {
    next_state.set(EngineState::PlayerInput);
    //println!("turn_update_end: Fin de mise à jour des events. Retour au PlayerInput.");
}

fn turn_update_cancel(
    mut next_state: ResMut<NextState<EngineState>>,    
    mut ev_cancel: EventWriter<CancelPlayerPendingActionsEvent>,    
) {    
    ev_cancel.send(CancelPlayerPendingActionsEvent); 
    next_state.set(EngineState::PlayerInput);
    //println!("turn_update_cancel: L'event du joueur a été rejeté. Retour au PlayerInput.");
}
