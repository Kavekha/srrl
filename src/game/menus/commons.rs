// Part of the refacto: what is commons to all menus
// TEMP

//Resume (ig menu) not working
//Music de combat reste quand retour au main menu depuis IG Menu.
//Reverifier coherence des menus.

//== DOCUMENTATION
// On commence en MenuState:Splashscreen, GameState:Disabled.
// Transmets un MessageEvent pour Afficher le Main Menu. Cet Event passe MenuState en Open.
// Le button_system contient tous les ordres de circulation dans les Menus, et est disponible dés que MenuState::Open.
// La gestion du ClearMenu reste pas ouf, il faut bien penser à l'ajouter à chaque fois. La mettre dans le Open semble faire le Clear après l'envoi du Menu... -_-

//== TODO 
// Encore beaucoup de doublons entre IG & MainMenu, à cause de la circulation. Peut être enregistrer l'option "Previous" dans le Menu à chaque fois?
// Desactiver les Controles "IG" / mettre GameState en Unavailable pendant le IG menu.


use bevy::prelude::*;

use crate::{
    engine::{audios::AudioType, save_load_system::has_save_file},
    game::{
        manager::{
            audio_messages::{ChangeMusicVolumeMessage, ChangeSoundVolumeMessage}, change_state_messages::{ChangeGameStateRunningMessage, ChangeGameStateUnavailableMessage, QuitGameMessage}, game_messages::{ClearGameMessage, StartGameMessage}, menu_messages::{ClearMenuMessage, CloseMenuMessage, InGameMenuQuitMessage, InGameMenuSettingsOpenMessage, InGameSettingsAudioMessage, InGameSettingsDisplayMessage, MainMenuOpenMessage, MainMenuQuitMessage, MainMenuSettingsAudioMessage, MainMenuSettingsDisplayMessage, MainMenuSettingsMessage, OpenInGameMenuOpenMessage}, save_messages::{LoadGameRequestMessage, SaveGameRequestMessage}, ExitAppMessage, MessageEvent
        },
        states::{GameState, MenuState}}, globals::{HEIGHT, RESOLUTION}
    };

use super::{button_system, components::{MenuButtonAction, ResolutionSettings}, menu_camera};

pub struct CommonsMenuPlugin;

impl Plugin for CommonsMenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ResolutionSettings{
                low:Vec2::new(640.0, 360.0),
                medium:Vec2::new(HEIGHT * RESOLUTION, HEIGHT),
                high:Vec2::new(1920.0, 1080.0)
            })
            
            .init_state::<MenuState>()
  
            //On fait ça pour utiliser la commande d'envoi du MainMenu, car on ne peut se baser sur le MenuState::Open.
            .add_systems(OnEnter(MenuState::Splashscreen), splashscreen)    
            .add_systems(OnEnter(MenuState::Splashscreen), menu_camera)  

            //.add_systems(OnEnter(MenuState::Open), menu_camera)
            .add_systems(Update, button_system.run_if(not(in_state(MenuState::Disabled))))
            .add_systems(Update, common_menu_action.run_if(not(in_state(MenuState::Disabled))))  // La gestion des actions IG Menu.
                 
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

// GameState is Running, I can call Menu.
pub fn ig_call_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Call for In Game Menu.");        
        ev_message.send(MessageEvent(Box::new(OpenInGameMenuOpenMessage))); 
        ev_message.send(MessageEvent(Box::new(ChangeGameStateUnavailableMessage))); 
    }
}

// GameState is Unavailable, I can close the menu.
pub fn ig_inside_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Back to game.");
        ev_message.send(MessageEvent(Box::new(ChangeGameStateRunningMessage))); 
        ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));         
    }
}

// Ne contient que MainMenu pour le moment.
pub fn common_menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
    mut ev_message: EventWriter<MessageEvent>
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
                    if has_save_file() {
                    println!("Load a saved game!");
                                         
                    ev_message.send(MessageEvent(Box::new(ClearGameMessage)));          // On efface si deja un jeu existant.
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(LoadGameRequestMessage)));        
                    } else {
                        println!("WARNING: No saved game.");
                    }          
                }
                MenuButtonAction::MainMenuSettings => {
                    println!("Main Menu Settings!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsMessage)));                     
                }
                MenuButtonAction::BackToMainMenu => {
                    println!("Back to Main Menu.");
                    ev_message.send(MessageEvent(Box::new(SaveGameRequestMessage)));
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuOpenMessage)));                     
                    ev_message.send(MessageEvent(Box::new(QuitGameMessage)));  
                }
                MenuButtonAction::MainMenuSettingsDisplay => {
                    println!("Main Menu Display Menu!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsDisplayMessage))); 
                }
                MenuButtonAction::MainMenuSettingsAudio => {
                    println!("Main Menu Audio  Menu!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsAudioMessage))); 
                }
                MenuButtonAction::SettingsAudioChange{modify_volume_by, audio_type} => {    //}, mut original_volume} => {
                    println!("Change volume for sound or music");
                    match audio_type {
                        AudioType::Sound => ev_message.send(MessageEvent(Box::new(ChangeSoundVolumeMessage { modify_value:modify_volume_by.clone()}))),
                        AudioType::Music => ev_message.send(MessageEvent(Box::new(ChangeMusicVolumeMessage { modify_value:modify_volume_by.clone()})))
                    };                     
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(MainMenuSettingsAudioMessage))); 
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
                MenuButtonAction::Close => {
                    println!("Close IG Menu");
                    ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));  
                    ev_message.send(MessageEvent(Box::new(ChangeGameStateRunningMessage)));  
                                    
                }
                MenuButtonAction::InGameMenuSettings => {
                    println!("IG Menu Setting");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(InGameMenuSettingsOpenMessage)));                  
                }
                MenuButtonAction::BackToInGameMenu => {
                    println!("Back to IG Menu");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(OpenInGameMenuOpenMessage)));                  
                }
                MenuButtonAction::InGameMenuQuit => {
                    println!("Want to quit?");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage)));                   
                    ev_message.send(MessageEvent(Box::new(InGameMenuQuitMessage)));                  
                }
                MenuButtonAction::InGameMenuDisplay => {
                    println!("Back to IG Menu");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(InGameSettingsDisplayMessage)));                  
                }
                MenuButtonAction::InGameMenuAudio => {
                    println!("Main Menu Audio  Menu!");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    ev_message.send(MessageEvent(Box::new(InGameSettingsAudioMessage))); 
                }
            }
        }
    }
}

