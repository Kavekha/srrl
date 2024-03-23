use bevy::ecs::{schedule::NextState, world::World};

use crate::game::{menus::components::InGameMenuState, states::MainMenuState};

use super::Message;


pub struct OpenMenuMessage;
impl Message for OpenMenuMessage {
    fn execute(&self, world: &mut World) {
        println!("End Game Recap?");
        if let Some(mut state) = world.get_resource_mut::<NextState<MainMenuState>>() {
            state.set(MainMenuState::RecapMenu);
            println!("yes");
        } else {
            println!("no");
        }
    }
}

pub struct CloseInGameMenuMessage;
impl Message for CloseInGameMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<InGameMenuState>>() {
            state.set(InGameMenuState::Disabled);
        }
    }
}

pub struct CloseMainMenuMessage;
impl Message for CloseMainMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<MainMenuState>>() {
            state.set(MainMenuState::Disabled);
        }
    }
}


pub struct ActiveMainMenuMessage;
impl Message for ActiveMainMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<MainMenuState>>() {
            state.set(MainMenuState::MainMenu);
        }
    }
}
pub struct ActiveInGameMenuMessage;
impl Message for ActiveInGameMenuMessage {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<InGameMenuState>>() {
            state.set(InGameMenuState::MainMenu);
        }
    }
}
