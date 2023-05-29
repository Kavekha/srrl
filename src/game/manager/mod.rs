use bevy::prelude::*;

use crate::{states::{GameState, EngineState}, render::GraphicsWaitEvent};

use super::{player::PlayerInputReadyEvent, actions::{TickEvent, ActionsCompleteEvent, InvalidPlayerActionEvent}};


pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameMap), game_start) 
            .add_systems(OnExit(GameState::GameMap), game_end)  

            .add_systems(Update, turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()))  
            .add_systems(Update, turn_update_end.run_if(on_event::<ActionsCompleteEvent>()))
            .add_systems(Update, turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()))
            .add_systems(OnEnter(EngineState::TurnUpdate), tick)
            .add_systems(Update, tick.run_if(in_state(EngineState::TurnUpdate)))
            ;
    }
}

fn game_start(
    mut next_state: ResMut<NextState<EngineState>>,
) {
    next_state.set(EngineState::PlayerInput);
    println!("game_start: Engine set to PlayerInput");
}

fn game_end(
    mut next_state: ResMut<NextState<EngineState>>
) {
    next_state.set(EngineState::None);
    println!("game_end: Engine set to None.")
}

fn turn_update_start(
    mut next_state: ResMut<NextState<EngineState>>,
    mut ev_tick: EventWriter<TickEvent>
) {
    next_state.set(EngineState::TurnUpdate);
    ev_tick.send(TickEvent);
    println!("turn_update_start ! Let's Send Tick and see if there is anything.")
}

fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>
) {
    if ev_wait.iter().len() == 0 {
        ev_tick.send(TickEvent);
        println!("tick: Everything is done. Tick suivant!");
    }
    println!("tick: ev_wait in process... {:?} to go.", ev_wait.iter().len());
}

fn turn_update_end(
    mut next_state: ResMut<NextState<EngineState>>
) {
    next_state.set(EngineState::PlayerInput);
    println!("turn_update_end: Fin de mise à jour des events. Retour au PlayerInput.");
}

fn turn_update_cancel(
    mut next_state: ResMut<NextState<EngineState>>
) {
    next_state.set(EngineState::PlayerInput);
    println!("turn_update_cancel: L'event du joueur a été rejeté. Retour au PlayerInput.");
}
