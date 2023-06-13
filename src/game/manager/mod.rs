use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{states::{GameState, EngineState, TurnSet}, render::GraphicsWaitEvent};

use super::{player::PlayerActionEvent, actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent, PlayerActions, ActorQueue}};


pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameMap), game_start) 
            .add_systems(OnExit(GameState::GameMap), game_end)  

            .configure_set(Update, TurnSet::Logic.run_if(in_state(EngineState::TurnUpdate)).before(TurnSet::Animation))   
            .configure_set(Update, TurnSet::Animation.run_if(in_state(EngineState::TurnUpdate)).before(TurnSet::Tick))   
            .configure_set(Update, TurnSet::Tick.run_if(in_state(EngineState::TurnUpdate)))   

            .add_systems(OnEnter(EngineState::PlayerInput), turn_player_pending_actions)
            .add_systems(Update, turn_update_start.run_if(on_event::<PlayerActionEvent>()))
            //.add_systems(Update, turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()))  

            .add_systems(Update, turn_update_end.run_if(on_event::<ActionsCompleteEvent>()))
            .add_systems(Update, turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()))
            .add_systems(OnEnter(EngineState::TurnUpdate), tick)
            .add_systems(Update, tick.run_if(in_state(EngineState::TurnUpdate)).in_set(TurnSet::Tick))
            ;
    }
}

fn turn_player_pending_actions(
    mut player_queue: ResMut<PlayerActions>,  
    mut queue: ResMut<ActorQueue>,
    mut ev_action: EventWriter<PlayerActionEvent>,  
){
    if let Some(entity) = player_queue.0.pop_front() {
        println!("turn player: an action is waiting for {:?}", entity);
        queue.0 = VecDeque::from([entity]);
        ev_action.send(PlayerActionEvent);
        println!("Sent: PlayerActionEvent");
    } else { 
        println!("turn player: No action waiting");
    };
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
    mut player_queue: ResMut<PlayerActions>,  
    mut queue: ResMut<ActorQueue>,
    mut next_state: ResMut<NextState<EngineState>>,
    mut ev_tick: EventWriter<TickEvent>
) {
    /* 
    if let Some(entity) = player_queue.0.pop_front() {
        println!("turn player: an action is waiting for {:?}", entity);
        queue.0 = VecDeque::from([entity]);
        //ev_action.send(PlayerActionEvent);
    } else { 
        println!("turn player: No action waiting");
    };*/

    next_state.set(EngineState::TurnUpdate);
    ev_tick.send(TickEvent);
    println!("turn_update_start by PlayerActionEvent! Let's Send Tick and see if there is anything.")
}

fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>
) {
    if ev_wait.iter().len() == 0 {
        ev_tick.send(TickEvent);
        //println!("tick: Waiting done. Tick suivant!");
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
    mut next_state: ResMut<NextState<EngineState>>
) {
    next_state.set(EngineState::PlayerInput);
    //println!("turn_update_cancel: L'event du joueur a été rejeté. Retour au PlayerInput.");
}
