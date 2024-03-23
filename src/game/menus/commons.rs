// Part of the refacto: what is commons to all menus
// TEMP

//Resume (ig menu) not working
//Music de combat reste quand retour au main menu depuis IG Menu.
//Reverifier coherence des menus.
// Les Actions devraient être dans le menuBuilder plutot
// Avec un seul Menu State, impossible de savoir lequel est souhaité: In Game affichera donc le MainMenu à ce stade.


use bevy::prelude::*;

use crate::game::{
    manager::{game_messages::{QuitGameMessage, StartGameMessage}, menu_messages::{ActiveInGameMenuMessage, ActiveMainMenuMessage, CloseInGameMenuMessage, CloseMainMenuMessage}, ExitAppMessage, MessageEvent}, 
    states::{GameState, MainMenuState}};

use super::{button_system, components::{MenuButtonAction, DisplayQuality, InGameMenuState, ResolutionSettings}, ingamemenu::{ig_call_menu_input, ig_inside_menu_input}, menu_camera};

pub struct CommonsMenuPlugin;

impl Plugin for CommonsMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<MenuState>()

            //.add_systems(Update, menu_tick.run_if(in_state(InGameMenuState::MainMenu)))     // Do it in event. poc.
            //.add_systems(Update, on_event_menu.run_if(on_event::<MenuEvent>()))             // Do it in event. poc.

             //Rassemblement Main Menu / IG MEnu : All actions. A la fin, devrait plutot se faire dans le MenuBuilder, associé à l'action
            .add_systems(OnEnter(MenuState::Open), menu_camera)
            .add_systems(Update, button_system.run_if(not(in_state(MenuState::Disabled))))
            .add_systems(Update, common_menu_action.run_if(not(in_state(MenuState::Disabled)))     )  // La gestion des actions IG Menu.
                 
            //Specific IG Menu            
            .add_systems(Update, ig_call_menu_input.run_if(in_state(GameState::Running)))   // Appeler IG Menu si In Game.            
            .add_systems(Update, ig_inside_menu_input.run_if(in_state(GameState::Unavailable)))     // TODO : Put the game In Unavailable quand Menu Open
        
            //Event system
            .add_event::<MenuEvent>()   
            .add_systems(Update, on_event_menu.run_if(in_state(MenuState::Open)).run_if(on_event::<MenuEvent>()))     
            ;
        }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Open,
    Disabled,
    //RecapMenu
}

#[derive(Event)]
pub enum MenuEvent {
    MainMenu,
    Settings,
    QuitConfirm,
    Close,
    Quit
}

fn on_event_menu(
    mut event_reader: EventReader<MenuEvent>
){
    println!("Processing Menu Event....");
    for event in event_reader.read() {
        match event {            
            MenuEvent::QuitConfirm => println!("Voulez vous vraiment quitter?"),
            MenuEvent::Close => println!("Closing Menu"),
            _ => println!("Menu Event non géré.")
        }
    }
}


// Ne contient que MainMenu pour le moment.
pub fn common_menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
     mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
    mut ev_message: EventWriter<MessageEvent>   ,
    mut ig_menu_state: ResMut<NextState<InGameMenuState>>,
    mut ev_writer: EventWriter<MenuEvent>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                //MainMenu is cop / pasta there
                MenuButtonAction::QuitConfirm => {
                    //app_exit_events.send(AppExit);
                    println!("Do you want to quit?");
                    menu_state.set(MainMenuState::QuitConfirm);    
                    ev_writer.send(MenuEvent::QuitConfirm);   //0.15.2                
                }
                MenuButtonAction::Quit => {
                    //app_exit_events.send(AppExit);
                    println!("Quit App");
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //app_exit_events.send(AppExit);
                    ev_writer.send(MenuEvent::Quit);   //0.15.2   
                }
                MenuButtonAction::Cancel => {
                    //app_exit_events.send(AppExit);
                    println!("Don't want to quit.");
                    ev_message.send(MessageEvent(Box::new(ActiveMainMenuMessage))); //menu_state.set(MainMenuState::MainMenu);
                    ev_writer.send(MenuEvent::MainMenu);   //0.15.2 
                }
                MenuButtonAction::Play => {
                    println!("Go to game !");                    
                    ev_message.send(MessageEvent(Box::new(CloseMainMenuMessage)));  //menu_state.set(MainMenuState::Disabled);
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //game_state.set(GameState::NewGame); 
                    ev_writer.send(MenuEvent::Close);   //0.15.2                 
                }
                MenuButtonAction::Load => {
                    println!("Load a saved game!");
                    //load_saved_game(&mut app_state, &mut game_state); 
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //menu_state.set(MainMenuState::Disabled);
                    game_state.set(GameState::LoadGame);
                    //load_game(app_state, game_state);
                    ev_writer.send(MenuEvent::Close);   //0.15.2 
                }
                MenuButtonAction::Settings => {
                    println!("Settings!");
                    menu_state.set(MainMenuState::Settings);
                    ev_writer.send(MenuEvent::Settings);   //0.15.2 
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to main menu");
                    ev_message.send(MessageEvent(Box::new(ActiveMainMenuMessage))); //menu_state.set(MainMenuState::MainMenu)
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::SettingsDisplay => {
                    println!("Display Menu!");
                    menu_state.set(MainMenuState::DisplayMenu);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings!");
                    menu_state.set(MainMenuState::Settings);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::DisplayLow => {
                    println!("Resolution changed to Low");
                    let mut window = windows.single_mut();                
                    let res = resolution.low;
                    window.resolution.set(res.x, res.y);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::DisplayMedium => {
                    println!("Resolution changed to Medium");
                    let mut window = windows.single_mut();              
                    let res = resolution.medium;
                    window.resolution.set(res.x, res.y);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::DisplayHigh => {
                    println!("Resolution changed to High");
                    let mut window = windows.single_mut();                  
                    let res = resolution.high;
                    window.resolution.set(res.x, res.y);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                // Ig Menu is cop / paste there
                MenuButtonAction::QuitConfirm => {
                    println!("Do you want to quit?");
                    ig_menu_state.set(InGameMenuState::QuitConfirm);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::Quit => {
                    println!("Quit App");
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage))); 
                }
                MenuButtonAction::Cancel => {
                    println!("Don't want to quit.");
                    ig_menu_state.set(InGameMenuState::MainMenu);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::BackToGame => {
                    println!("Go to game !");
                    ev_message.send(MessageEvent(Box::new(CloseMainMenuMessage)));    //menu_state.set(InGameMenuState::Disabled);
                    ev_writer.send(MenuEvent::Close);   //POC                  
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to main menu");
                    ev_message.send(MessageEvent(Box::new(QuitGameMessage)));   // game_state.set(GameState::Disabled);
                    ev_message.send(MessageEvent(Box::new(CloseInGameMenuMessage)));     //menu_state.set(InGameMenuState::Disabled);   
                    ev_message.send(MessageEvent(Box::new(ActiveMainMenuMessage)));   //main_menu_state.set(MainMenuState::MainMenu);    
                    ev_writer.send(MenuEvent::Close);   //POC             
                }
                MenuButtonAction::Settings => {
                    println!("Go to Settings");
                    ig_menu_state.set(InGameMenuState::Settings);   
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::DisplayLow => {
                    println!("Change to Low");
                    let mut window = windows.single_mut();
                    let res = resolution.low;
                    window.resolution.set(res.x, res.y); 
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::DisplayHigh => {
                    println!("Change to High");
                    let mut window = windows.single_mut();                    
                    let res = resolution.high;
                    window.resolution.set(res.x, res.y);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::DisplayMedium => {
                    println!("Change to Medium");     
                    let mut window = windows.single_mut();                                   
                    let res = resolution.medium;
                    window.resolution.set(res.x, res.y);
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                MenuButtonAction::SettingsDisplay => {
                    println!("Go to Settings Display");
                    ig_menu_state.set(InGameMenuState::SettingDisplay); 
                    ev_writer.send(MenuEvent::Close);   //POC  
                }
                MenuButtonAction::Back => {
                    println!("Go back to Menu");
                    ev_message.send(MessageEvent(Box::new(ActiveInGameMenuMessage))); //menu_state.set(InGameMenuState::MainMenu);  
                    ev_writer.send(MenuEvent::Close);   //POC 
                }
                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings");
                    ig_menu_state.set(InGameMenuState::Settings);   
                    ev_writer.send(MenuEvent::Close);   //POC
                }
                _ => {
                    println!("Something Else to deal with!");
                }
            }
        }
    }
}
