// Part of the refacto: what is commons to all menus
// TEMP

//Resume (ig menu) not working
//Music de combat reste quand retour au main menu depuis IG Menu.
//Reverifier coherence des menus.

//== DOCUMENTATION
// On commence en MenuState:Splashscreen, GameState:Disabled.
// Transmets un MessageEvent pour Afficher le Main Menu. Cet Event passe MenuState en Open.
// Le button_system contient tous les ordres de circulation dans les Menus, et est disponible dés que MenuState::Open.

//== BUGS & AMELIORATIONS
// Display ne semble pas s'afficher tout le temps?

use bevy::prelude::*;

use crate::game::{
    manager::{game_messages::{QuitGameMessage, StartGameMessage}, menu_messages::{ActiveInGameMenuMessage, ActiveMainMenuMessage, ClearMenuMessage, CloseInGameMenuMessage, CloseMainMenuMessage, CloseMenuMessage, MainMenuOpenMessage, MainMenuQuitMessage, MainMenuSettingsDisplayMessage, MainMenuSettingsMessage}, ExitAppMessage, MessageEvent}, 
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
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage)));          
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));              
                }
                //TODO : Reactive LOAD.
                MenuButtonAction::Load => {
                    println!("Load a saved game!");
                    //load_saved_game(&mut app_state, &mut game_state); 
                    //load_game(app_state, game_state); 
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));      // NEW MESSAGE EVENT SYSTEM v0.15.2 //menu_state.set(MainMenuState::Disabled);             
                }
                MenuButtonAction::MainMenuSettings => {
                    println!("Main Menu Settings!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsMessage)));                     
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to Main Menu.");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuOpenMessage))); 
                }
                MenuButtonAction::MainMenuSettingsDisplay => {
                    println!("Main Menu Display Menu!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsDisplayMessage))); 
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
                MenuButtonAction::MainMenuBackToSettings => {
                    println!("Main Menu Display Menu!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsDisplayMessage))); 
                }
                MenuButtonAction::Quit => {
                    println!("Do you want to quit?");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuQuitMessage)));                     
                }                
                MenuButtonAction::QuitConfirm => {
                    println!("Quit App");
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage)));                  
                }
                //Specific In Game Menu.


//MainMenu is cop / pasta there


                MenuButtonAction::BackToSettings => {
                    println!("Back to Settings!");
                    menu_state.set(MainMenuState::Settings);
                }

                // Ig Menu is cop / paste there
                MenuButtonAction::QuitConfirm => {
                    println!("Do you want to quit?");
                    ig_menu_state.set(InGameMenuState::QuitConfirm);
                }
                MenuButtonAction::QuitConfirm => {
                    println!("Quit App");
                    ev_message.send(MessageEvent(Box::new(ExitAppMessage))); 
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
