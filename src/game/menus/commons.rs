// Part of the refacto: what is commons to all menus
// TEMP

//Resume (ig menu) not working
//Music de combat reste quand retour au main menu depuis IG Menu.
//Reverifier coherence des menus.
// Les Actions devraient être dans le menuBuilder plutot
// Avec un seul Menu State, impossible de savoir lequel est souhaité: In Game affichera donc le MainMenu à ce stade.

//== DOCUMENTATION
// On commence en MenuState:Splashscreen, GameState:Disabled.
// Transmets un MessageEvent pour Afficher le Main Menu. Cet Event passe MenuState en Open.
// Le button_system contient tous les ordres de circulation dans les Menus, et est disponible dés que MenuState::Open.

use bevy::prelude::*;

use crate::game::{
    manager::{game_messages::{QuitGameMessage, StartGameMessage}, menu_messages::{ActiveInGameMenuMessage, ActiveMainMenuMessage, CloseInGameMenuMessage, CloseMainMenuMessage, CloseMenuMessage, MainMenuOpenMessage, MainMenuSettingsDisplayMessage, MainMenuSettingsMessage}, ExitAppMessage, MessageEvent}, 
    states::{GameState, MainMenuState, MenuState}};

use super::{button_system, components::{MenuButtonAction, DisplayQuality, InGameMenuState, ResolutionSettings}, ingamemenu::{ig_call_menu_input, ig_inside_menu_input}, menu_camera};

pub struct CommonsMenuPlugin;

impl Plugin for CommonsMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<MenuState>()
  
            .add_systems(OnEnter(MenuState::Splashscreen), splashscreen)    
            .add_systems(OnEnter(MenuState::Splashscreen), menu_camera)  

             //Rassemblement Main Menu / IG MEnu : All actions. A la fin, devrait plutot se faire dans le MenuBuilder, associé à l'action
            .add_systems(OnEnter(MenuState::Open), menu_camera)
            .add_systems(Update, button_system.run_if(not(in_state(MenuState::Disabled))))
            .add_systems(Update, common_menu_action.run_if(not(in_state(MenuState::Disabled)))     )  // La gestion des actions IG Menu.
                 
            //Specific IG Menu            
            .add_systems(Update, ig_call_menu_input.run_if(in_state(GameState::Running)))   // Appeler IG Menu si In Game.            
            .add_systems(Update, ig_inside_menu_input.run_if(in_state(GameState::Unavailable)))     // TODO : Put the game In Unavailable quand Menu Open
            ;
        }
}



fn splashscreen(
    mut ev_message: EventWriter<MessageEvent>
){
    println!("Splashscreen: Open Main Menu");
    ev_message.send(MessageEvent(Box::new(MainMenuOpenMessage)));
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
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    println!("Go to game !");                    
                    ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));  
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));              
                }
                //TODO : Reactive LOAD.
                MenuButtonAction::Load => {
                    println!("Load a saved game!");
                    //load_saved_game(&mut app_state, &mut game_state); 
                    //load_game(app_state, game_state);
                    ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));  
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //menu_state.set(MainMenuState::Disabled);             
                }
                MenuButtonAction::Settings => {
                    println!("Settings!");
                    ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));  
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsMessage)));                     
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to Main Menu.");
                    ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));  
                    ev_message.send(MessageEvent(Box::new(MainMenuOpenMessage))); 
                }



                //MainMenu is cop / pasta there
                MenuButtonAction::QuitConfirm => {
                    //app_exit_events.send(AppExit);
                    println!("Do you want to quit?");
                    menu_state.set(MainMenuState::QuitConfirm);                  
                }
                MenuButtonAction::Quit => {
                    //app_exit_events.send(AppExit);
                    println!("Quit App");
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //app_exit_events.send(AppExit);
                }
                MenuButtonAction::Cancel => {
                    //app_exit_events.send(AppExit);
                    println!("Don't want to quit.");
                    ev_message.send(MessageEvent(Box::new(ActiveMainMenuMessage))); //menu_state.set(MainMenuState::MainMenu);
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to main menu");
                    ev_message.send(MessageEvent(Box::new(ActiveMainMenuMessage))); //menu_state.set(MainMenuState::MainMenu)
                }
                MenuButtonAction::SettingsDisplay => {
                    println!("Display Menu!");
                    menu_state.set(MainMenuState::DisplayMenu);
                }
                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings!");
                    menu_state.set(MainMenuState::Settings);
                }
                MenuButtonAction::DisplayLow => {
                    println!("Resolution changed to Low");
                    let mut window = windows.single_mut();                
                    let res = resolution.low;
                    window.resolution.set(res.x, res.y);
                }
                MenuButtonAction::DisplayMedium => {
                    println!("Resolution changed to Medium");
                    let mut window = windows.single_mut();              
                    let res = resolution.medium;
                    window.resolution.set(res.x, res.y);
                }
                MenuButtonAction::DisplayHigh => {
                    println!("Resolution changed to High");
                    let mut window = windows.single_mut();                  
                    let res = resolution.high;
                    window.resolution.set(res.x, res.y);
                }
                // Ig Menu is cop / paste there
                MenuButtonAction::QuitConfirm => {
                    println!("Do you want to quit?");
                    ig_menu_state.set(InGameMenuState::QuitConfirm);
                }
                MenuButtonAction::Quit => {
                    println!("Quit App");
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage))); 
                }
                MenuButtonAction::Cancel => {
                    println!("Don't want to quit.");
                    ig_menu_state.set(InGameMenuState::MainMenu);
                }
                MenuButtonAction::BackToGame => {
                    println!("Go to game !");
                    ev_message.send(MessageEvent(Box::new(CloseMainMenuMessage)));    //menu_state.set(InGameMenuState::Disabled);             
                }
                MenuButtonAction::Settings => {
                    println!("Go to Settings");
                    ig_menu_state.set(InGameMenuState::Settings); 
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
                    ig_menu_state.set(InGameMenuState::SettingDisplay); 
                }
                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings");
                    ig_menu_state.set(InGameMenuState::Settings);   
                }
                _ => {
                    println!("Something Else to deal with!");
                }
            }
        }
    }
}
