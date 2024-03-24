use bevy::ecs::{schedule::NextState, world::World};

use crate::game::states::GameState;

use super::{game_messages::ClearGameMessage, Message, MessageEvent};


pub struct ChangeGameStateInitialiseRequestMessage;
impl Message for ChangeGameStateInitialiseRequestMessage {
    fn execute(&self, world: &mut World) {
        println!("Initialising game, for Running State.");
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Initialise);
        }
        world.send_event(MessageEvent(Box::new(ChangeGameStateRunningMessage)));
    }
}


pub struct ChangeGameStateInitialiseMessage;
impl Message for ChangeGameStateInitialiseMessage {
    fn execute(&self, world: &mut World) {
        println!("Game State is now Initialise.");
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Initialise);
        }        
    }
}


pub struct ChangeGameStateRunningMessage;
impl Message for ChangeGameStateRunningMessage {
    fn execute(&self, world: &mut World) {
        println!("Game State is now Running.");
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Running);
        }
    }
}

pub struct ChangeGameStateProcessingMessage;
impl Message for ChangeGameStateProcessingMessage {
    fn execute(&self, world: &mut World) {
        println!("Game State is now Processing.");    
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Processing);
        }
    }
}

pub struct ChangeGameStateDisabledMessage;
impl Message for ChangeGameStateDisabledMessage {
    fn execute(&self, world: &mut World) {
        println!("Game State is now Disabled.");    
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::Disabled);
        }
    }
}

pub struct QuitGameMessage;
impl Message for QuitGameMessage {
    fn execute(&self, world: &mut World) {
        world.send_event(MessageEvent(Box::new(ClearGameMessage)));
        world.send_event(MessageEvent(Box::new(ChangeGameStateDisabledMessage)));
    }
}