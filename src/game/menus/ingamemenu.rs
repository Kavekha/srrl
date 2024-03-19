use bevy::{prelude::*, app::AppExit};

//use crate::engine::states::AppState;

use super::{clean_menu, components::{DisplayQuality, InGameMenuState, ResolutionSettings}, menu_camera, button_system};

use crate::{game::{menus::menu_builder::{spawn_basic_menu, Menu, MenuView}, states::{GameState, MainMenuState}}, globals::{NORMAL_BUTTON, TEXT_COLOR}};

use super::components::{MenuButtonAction, OnScreenMenu} 
;


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

            .add_systems(Update, ig_call_menu_input.run_if(in_state(InGameMenuState::Disabled)))    // TODO : Peut quand meme etre appelé du Main Menu -_-
            .add_systems(Update, ig_inside_menu_input.run_if(in_state(InGameMenuState::MainMenu)))  //TODO : Not Disabled

            .add_systems(OnEnter(InGameMenuState::MainMenu), menu_camera) 
            .add_systems(OnEnter(InGameMenuState::MainMenu), enter_ig_main_menu)
            .add_systems(OnEnter(InGameMenuState::Settings), enter_ig_settings_menu)
            .add_systems(OnEnter(InGameMenuState::SettingDisplay), enter_ig_display_menu) 

            //Todo with not in Disable?
            .add_systems(Update, button_system.run_if(not(in_state(InGameMenuState::Disabled))))
            .add_systems(Update, ig_menu_action.run_if(not(in_state(InGameMenuState::Disabled)))     )
            
            .add_systems(OnExit(InGameMenuState::MainMenu), clean_menu)
            .add_systems(OnExit(InGameMenuState::Settings), clean_menu)
            .add_systems(OnExit(InGameMenuState::SettingDisplay), clean_menu)               
            .add_systems(OnExit(InGameMenuState::QuitConfirm), clean_menu)
            ;
    }
}
 
// Do it in event. poc.
/* 
#[derive(Event)]
pub enum MenuEvent {
    Close,
}
// Do it in event. poc.
fn menu_tick(
    mut ev_writer: EventWriter<MenuEvent>
){
    println!("Tick!");
    ev_writer.send(MenuEvent::Close);
}
// Do it in event. poc.
fn on_event_menu(
    mut event_reader: EventReader<MenuEvent>
){
    for event in event_reader.read() {
        match event {
            MenuEvent::Close => println!("Closing Menu")
        }
    }
    println!("Processing Menu Event....");
}
*/

pub fn ig_call_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<InGameMenuState>>
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Call for In Game Menu.");
        menu_state.set(InGameMenuState::MainMenu);
    }
}

pub fn ig_inside_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<NextState<InGameMenuState>>
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Back to game.");
        menu_state.set(InGameMenuState::Disabled);
    }
}


pub fn ig_menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
    mut app_exit_events: EventWriter<AppExit>,
    //mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<InGameMenuState>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::QuitConfirm => {
                    println!("Do you want to quit?");
                    menu_state.set(InGameMenuState::QuitConfirm);
                }
                MenuButtonAction::Quit => {
                    println!("Quit App");
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Cancel => {
                    println!("Don't want to quit.");
                    menu_state.set(InGameMenuState::MainMenu);
                }
                MenuButtonAction::BackToGame => {
                    println!("Go to game !");
                    menu_state.set(InGameMenuState::Disabled);                  
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to main menu");
                    main_menu_state.set(MainMenuState::MainMenu);
                    menu_state.set(InGameMenuState::Disabled);   
                    game_state.set(GameState::Disabled);
                }
                MenuButtonAction::Settings => {
                    println!("Go to Settings");
                    menu_state.set(InGameMenuState::Settings);   
                }
                MenuButtonAction::DisplayLow => {
                    println!("Change to Low");
                    let mut window = windows.single_mut();
                    let res = resolution.low;
                    window.resolution.set(res.x, res.y); 
                }
                MenuButtonAction::DisplayHigh => {
                    println!("Change to High");
                    let mut window = windows.single_mut();                    
                    let res = resolution.high;
                    window.resolution.set(res.x, res.y);
                }
                MenuButtonAction::DisplayMedium => {
                    println!("Change to Medium");     
                    let mut window = windows.single_mut();                                   
                    let res = resolution.medium;
                    window.resolution.set(res.x, res.y);
                }
                MenuButtonAction::SettingsDisplay => {
                    println!("Go to Settings Display");
                    menu_state.set(InGameMenuState::SettingDisplay);   
                }
                MenuButtonAction::Back => {
                    println!("Go back to Menu");
                    menu_state.set(InGameMenuState::MainMenu);   
                }
                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings");
                    menu_state.set(InGameMenuState::Settings);   
                }
                _ => {
                    println!("Something Else to deal with!");
                }
            }
        }
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
