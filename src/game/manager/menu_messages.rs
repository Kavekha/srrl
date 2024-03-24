use bevy::ecs::{schedule::NextState, world::World};

use crate::game::{
    manager::MessageEvent, 
    menus::{clean_menu, components::MenuButtonAction, menu_builder::{MenuItem, MenuV2}, recapmenu::{MenuEvent, MenuType}}, states::MenuState};

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


// Open X Menu : Le MenuEvent doit être envoyé avant le OpenMenu car on fait un clean? ou alors les MenuEvent doivent être traité .after les MessagesEvents?

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

        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::MAINMENU});
        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("MainMenu generated and send for opening.");
    }
}

// Open MainMenuSettings
pub struct MainMenuSettingsMessage;
impl Message for MainMenuSettingsMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_settings", Vec::new());
        menu.add(MenuItem::description("Settings"));
        menu.add(MenuItem::action(MenuButtonAction::MainMenuSettingsDisplay, "Display"));
        menu.add(MenuItem::action(MenuButtonAction::BackToMainMenu, "Back"));

        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::SETTINGS});
        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("Settings generated and send for opening.");
    }
}

// Open MainMenuSettingsDisplay
pub struct MainMenuSettingsDisplayMessage;
impl Message for MainMenuSettingsDisplayMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_settings_display", Vec::new());
        menu.add(MenuItem::description("Choose your resolution"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayLow, "Low"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayMedium, "Medium"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayHigh, "High"));
        menu.add(MenuItem::action(MenuButtonAction::MainMenuSettings, "Back"));

        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::DISPLAY});
        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("SettingsDisplay generated and send for opening.");
    }
}

pub struct MainMenuQuitMessage;
impl Message for MainMenuQuitMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_quit", Vec::new());
        menu.add(MenuItem::description("Do you want to quit?"));
        menu.add(MenuItem::action(MenuButtonAction::BackToMainMenu, "Cancel"));
        menu.add(MenuItem::action(MenuButtonAction::QuitConfirm, "Confirm"));

         world.send_event(MenuEvent{menu:menu, menu_type:MenuType::QUIT});
         world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("Quit generated and send for opening.");
    }
}

pub struct OpenInGameMenuOpenMessage;
impl Message for OpenInGameMenuOpenMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("ig_menu", Vec::new());
        menu.add(MenuItem::action(MenuButtonAction::Close, "Resume"));
        menu.add(MenuItem::action(MenuButtonAction::Load, "Load game"));
        menu.add(MenuItem::action(MenuButtonAction::InGameMenuSettings, "Settings"));
        menu.add(MenuItem::action(MenuButtonAction::BackToMainMenu, "Main Menu"));
        menu.add(MenuItem::action(MenuButtonAction::InGameMenuQuit, "Quit"));

        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::MAINMENU});
        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("InGame Menu generated and send for opening.");
    }
}

pub struct InGameMenuSettingsOpenMessage;
impl Message for InGameMenuSettingsOpenMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("ig_menu_settings", Vec::new());
        menu.add(MenuItem::description("Settings"));
        menu.add(MenuItem::action(MenuButtonAction::InGameMenuDisplay, "Display"));
        menu.add(MenuItem::action(MenuButtonAction::BackToInGameMenu, "Back"));

        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::SETTINGS});
        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("InGame Menu generated and send for opening.");
    }
}

pub struct InGameMenuQuitMessage;
impl Message for InGameMenuQuitMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_quit", Vec::new());
        menu.add(MenuItem::description("Do you want to quit?"));
        menu.add(MenuItem::action(MenuButtonAction::BackToInGameMenu, "Cancel"));
        menu.add(MenuItem::action(MenuButtonAction::QuitConfirm, "Confirm"));

         world.send_event(MenuEvent{menu:menu, menu_type:MenuType::QUIT});
         world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("Quit generated and send for opening.");
    }
}

pub struct InGameSettingsDisplayMessage;
impl Message for InGameSettingsDisplayMessage {
    fn execute(&self, world: &mut World) {
        let mut menu = MenuV2::new("main_menu_settings_display", Vec::new());
        menu.add(MenuItem::description("Choose your resolution"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayLow, "Low"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayMedium, "Medium"));
        menu.add(MenuItem::action(MenuButtonAction::DisplayHigh, "High"));
        menu.add(MenuItem::action(MenuButtonAction::InGameMenuSettings, "Back"));

        world.send_event(MenuEvent{menu:menu, menu_type:MenuType::DISPLAY});
        world.send_event(MessageEvent(Box::new(OpenMenuMessage)));
        println!("SettingsDisplay generated and send for opening.");
    }
}



