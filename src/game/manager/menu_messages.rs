use bevy::ecs::{schedule::NextState, world::World};

use crate::game::{manager::MessageEvent, menus::{clean_menu, components::{InGameMenuState, MenuButtonAction}, menu_builder::{MenuItem, MenuV2}, recapmenu::{MenuEvent, MenuType}}, states::{MainMenuState, MenuState}};

use super::Message;


pub struct OpenMenuMessage;
impl Message for OpenMenuMessage {
    fn execute(&self, world: &mut World) {
        println!("OpenMenuMessage");
        if let Some(mut state) = world.get_resource_mut::<NextState<MenuState>>() {
            state.set(MenuState::Open);
        }
    }
}

pub struct CloseMenuMessage;
impl Message for CloseMenuMessage {
    fn execute(&self, world: &mut World) {
        println!("CloseMenuMessage");
        if let Some(mut state) = world.get_resource_mut::<NextState<MenuState>>() {
            state.set(MenuState::Disabled);
            world.send_event(MessageEvent(Box::new(ClearMenuMessage)));
        }
    }
}

pub struct ClearMenuMessage;
impl Message for ClearMenuMessage {
    fn execute(&self, world: &mut World) {
        let clean_menu = world.register_system(clean_menu);
        let result = world.run_system(clean_menu);
        println!("Clean menu result: {:?}", result);
    }
}


// Open MainMenu

pub struct MainMenuOpenMessage;
impl Message for MainMenuOpenMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu", Vec::new());
        menu.add(MenuItem::header("ShadowRun"));
        menu.add(MenuItem::description("v0.15.2 - R0.4"));
        menu.add(MenuItem::action(MenuButtonAction::Play, "Play"));
        menu.add(MenuItem::action(MenuButtonAction::Load, "Load game"));
        menu.add(MenuItem::action(MenuButtonAction::MainMenuSettings, "Settings"));
        menu.add(MenuItem::action(MenuButtonAction::Quit, "Quit"));

        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::MAINMENU});
        println!("MainMenu generated and send for opening.");
    }
}

// Open MainMenuSettings
pub struct MainMenuSettingsMessage;
impl Message for MainMenuSettingsMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_settings", Vec::new());
        menu.add(MenuItem::header("Settings"));
        menu.add(MenuItem::action(MenuButtonAction::MainMenuSettingsDisplay, "Display"));
        menu.add(MenuItem::action(MenuButtonAction::BackToMainMenu, "Back"));

        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::SETTINGS});
        println!("Settings generated and send for opening.");
    }
}

// Open MainMenuSettingsDisplay
pub struct MainMenuSettingsDisplayMessage;
impl Message for MainMenuSettingsDisplayMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_settings_display", Vec::new());
        menu.add(MenuItem::header("Display"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayLow, "Low"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayMedium, "Medium"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayHigh, "High"));
        menu.add(MenuItem::action(MenuButtonAction::MainMenuBackToSettings, "Back"));

        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::DISPLAY});
        println!("SettingsDisplay generated and send for opening.");
    }
}

pub struct MainMenuQuitMessage;
impl Message for MainMenuQuitMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_quit", Vec::new());
        menu.add(MenuItem::header("Display"));
        menu.add(MenuItem::action(MenuButtonAction::BackToMainMenu, "Cancel"));
        menu.add(MenuItem::action(MenuButtonAction::QuitConfirm, "Confirm"));

        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::QUIT});
        println!("Quit generated and send for opening.");
    }
}





// ==== OLD, to review.

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





