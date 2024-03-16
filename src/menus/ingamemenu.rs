use bevy::{prelude::*, app::AppExit};

use crate::states::AppState;

use super::{clean_menu, components::{DisplayQuality, ResolutionSettings}, mainmenu::{button_system, menu_action, menu_camera, resolution_menu_action}};

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<InGameMenuState>()

            .insert_resource(DisplayQuality::Medium)
            .insert_resource(ResolutionSettings{
                low:Vec2::new(640.0, 360.0),
                medium:Vec2::new(800.0, 600.0),
                high:Vec2::new(1920.0, 1080.0)
            })
            
            //.add_systems(OnEnter(AppState::MainMenu), load_main_menu)
            .add_systems(OnEnter(InGameMenuState::MainMenu), menu_camera)  
            //.add_systems(OnEnter(InGameMenuState::MainMenu), spawn_main_menu)      
            //.add_systems(OnEnter(InGameMenuState::Settings), spawn_settings_menu)      
            //.add_systems(OnEnter(InGameMenuState::DisplayMenu), spawn_display_menu)      
            //.add_systems(OnEnter(InGameMenuState::QuitConfirm), spawn_quit_confirm_menu)
            
            .add_systems(Update, call_for_ingame_menu_input.run_if(in_state(AppState::Game)))
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu)))   
            .add_systems(Update, resolution_menu_action.run_if(in_state(InGameMenuState::DisplayMenu)))    //Only in display menu there. Not really cool but hey.   
            

            .add_systems(OnExit(InGameMenuState::MainMenu), clean_menu)
            .add_systems(OnExit(InGameMenuState::Settings), clean_menu)
            .add_systems(OnExit(InGameMenuState::DisplayMenu), clean_menu)               
            .add_systems(OnExit(InGameMenuState::QuitConfirm), clean_menu);
            //.add_systems(OnExit(AppState::MainMenu), quit_main_menu);
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameMenuState {
    #[default]
    Disabled,
    MainMenu,
    Settings,
    DisplayMenu,
    QuitConfirm
}


pub fn call_for_ingame_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<InGameMenuState>>
){
    if keys.just_pressed(KeyCode::KeyU) {
        println!("Call for In Game Menu.");
    }
    println!("J'existe!");
    //mainmenu_state.set(MainMenuState::MainMenu);
}