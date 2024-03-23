use bevy::ecs::{schedule::NextState, world::World};

use crate::game::{menus::{clean_menu, components::InGameMenuState}, states::{MainMenuState, MenuState}};

use super::Message;


// v2 : was recap.
pub struct OpenMenuMessage;
impl Message for OpenMenuMessage {
    fn execute(&self, world: &mut World) {
        println!("End Game Recap?");
        if let Some(mut state) = world.get_resource_mut::<NextState<MenuState>>() {
            state.set(MenuState::Open);
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

// 0.15.2 tjrs dans les systems pour le moment.
pub struct ClearMenuMessage;
impl Message for ClearMenuMessage {
    fn execute(&self, world: &mut World) {
        let clean_menu = world.register_system(clean_menu);
        let result = world.run_system(clean_menu);
        println!("Clean menu result: {:?}", result);
    }
}