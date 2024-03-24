use bevy::prelude::*;

//use crate::engine::states::AppState;

use super::{clean_menu, components::{DisplayQuality, InGameMenuState, ResolutionSettings}, menu_camera, button_system};

use crate::game::{
    manager::{
        game_messages::QuitGameMessage, 
        menu_messages::{
            CloseMenuMessage, OpenInGameMenuOpenMessage
        }, 
        ExitAppMessage, MessageEvent},
    menus::menu_builder::{spawn_basic_menu, Menu, MenuView}
};   

use super::components::MenuButtonAction;    //, OnScreenMenu} 


pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<InGameMenuState>()

            //.add_event::<MenuEvent>()   // Do it in event. poc.

            .insert_resource(DisplayQuality::Medium)
            .insert_resource(ResolutionSettings{
                low:Vec2::new(640.0, 360.0),
                medium:Vec2::new(800.0, 600.0),
                high:Vec2::new(1920.0, 1080.0)
            })
            //.add_systems(Update, menu_tick.run_if(in_state(InGameMenuState::MainMenu)))     // Do it in event. poc.
            //.add_systems(Update, on_event_menu.run_if(on_event::<MenuEvent>()))             // Do it in event. poc.

            //.add_systems(Update, ig_call_menu_input.run_if(in_state(InGameMenuState::Disabled)))    // TODO : Peut quand meme etre appelé du Main Menu -_-
            //.add_systems(Update, ig_inside_menu_input.run_if(in_state(InGameMenuState::MainMenu)))  //TODO : Not Disabled

            //.add_systems(OnEnter(InGameMenuState::MainMenu), menu_camera) //0.15.2 in Commons
            .add_systems(OnEnter(InGameMenuState::MainMenu), enter_ig_main_menu)
            .add_systems(OnEnter(InGameMenuState::Settings), enter_ig_settings_menu)
            .add_systems(OnEnter(InGameMenuState::SettingDisplay), enter_ig_display_menu) 
            .add_systems(OnEnter(InGameMenuState::QuitConfirm), enter_ig_quit_confirm_menu)

            //Todo with not in Disable?
            //.add_systems(Update, button_system.run_if(not(in_state(InGameMenuState::Disabled))))      // 0.15.2 in Commons
            //.add_systems(Update, ig_menu_action.run_if(not(in_state(InGameMenuState::Disabled)))     )// 0.15.2 in Commons
            
            //ALl in one.
            //.add_systems(OnEnter(InGameMenuState::Disabled), clean_menu)
            ;
    }
}  
  
// GameState is Running, I can call Menu.
pub fn ig_call_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<InGameMenuState>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Call for In Game Menu.");
        //menu_state.set(InGameMenuState::MainMenu);
        ev_message.send(MessageEvent(Box::new(OpenInGameMenuOpenMessage))); 
    }
}

// GameState is Unavailable, I can close the menu.
pub fn ig_inside_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<InGameMenuState>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Back to game.");
        //menu_state.set(InGameMenuState::Disabled);
        ev_message.send(MessageEvent(Box::new(CloseMenuMessage))); 
    }
}




pub fn enter_ig_main_menu(mut commands: Commands) {
    println!("Entering IG Main menu.");
    let mut menu = Menu::new();
    for (action, text) in [
                        (MenuButtonAction::BackToGame, "Resume"),
                        (MenuButtonAction::Settings, "Settings"),
                        (MenuButtonAction::BackToMainMenu, "Main Menu"),
                        (MenuButtonAction::QuitConfirm, "Quit"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}

pub fn enter_ig_settings_menu(mut commands: Commands) {
    println!("Entering IG Setting Menu.");
    let mut menu = Menu::new();
    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        //(MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::Back, "Back"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}

pub fn enter_ig_display_menu(mut commands: Commands) {
    println!("Entering IG Display Menu.");
    let mut menu = Menu::new();
    for (action, text) in [
                        (MenuButtonAction::DisplayLow, "Low"),
                        (MenuButtonAction::DisplayMedium, "Medium"),
                        (MenuButtonAction::DisplayHigh, "High"),
                        (MenuButtonAction::BackToSettings, "Back"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}

pub fn enter_ig_quit_confirm_menu(mut commands: Commands) {
    println!("Entering IG Quit Confirm menu.");
    let mut menu = Menu::new();
    for (action, text) in [                            
            //(MenuButtonAction::Cancel, "Cancel"),
            (MenuButtonAction::Quit, "Confirm"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}