use bevy::ecs::{schedule::NextState, world::World};

use crate::game::states::GameState;

use super::Message;




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

