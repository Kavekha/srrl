use bevy::prelude::*;


use super::components::{DisplayQuality, ResolutionSettings};

use crate::game::manager::{menu_messages::{CloseMenuMessage, OpenInGameMenuOpenMessage}, MessageEvent};   



pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin{
    fn build(&self, app: &mut App) {
        app

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
            //.add_systems(OnEnter(InGameMenuState::MainMenu), enter_ig_main_menu)
            //.add_systems(OnEnter(InGameMenuState::Settings), enter_ig_settings_menu)
            //.add_systems(OnEnter(InGameMenuState::SettingDisplay), enter_ig_display_menu) 
            //.add_systems(OnEnter(InGameMenuState::QuitConfirm), enter_ig_quit_confirm_menu)

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
    //mut menu_state: ResMut<NextState<InGameMenuState>>,
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
    //mut menu_state: ResMut<NextState<InGameMenuState>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Back to game.");
        //menu_state.set(InGameMenuState::Disabled);
        ev_message.send(MessageEvent(Box::new(CloseMenuMessage))); 
    }
}
