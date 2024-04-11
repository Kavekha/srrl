
 
use bevy::prelude::*;

use crate::{
    engine::{audios::AudioType, save_load_system::has_save_file},
    game::{manager::{
            audio_messages::{ChangeMusicVolumeMessage, ChangeSoundVolumeMessage}, 
            change_state_messages::{ChangeGameStateRunningMessage, QuitGameMessage}, 
            game_messages::{ClearGameMessage, StartGameMessage},
            menu_messages::{ClearMenuMessage, CloseMenuMessage, InGameMenuQuitMessage, InGameMenuSettingsOpenMessage, InGameSettingsAudioMessage, InGameSettingsDisplayMessage, MainMenuOpenMessage, MainMenuQuitMessage, MainMenuSettingsAudioMessage, MainMenuSettingsDisplayMessage, MainMenuSettingsMessage, OpenInGameMenuOpenMessage}, 
            save_messages::{LoadGameRequestMessage, SaveGameRequestMessage}, 
            ExitAppMessage, MessageEvent
        }, states::GameState}
};

use super::components::{MenuButtonAction, ResolutionSettings};




pub fn splashscreen(
    mut ev_message: EventWriter<MessageEvent>
){
    println!("Splashscreen: Open Main Menu");
    ev_message.send(MessageEvent(Box::new(MainMenuOpenMessage)));
}



// Ne contient que MainMenu pour le moment.
pub fn common_menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>),>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
    mut ev_message: EventWriter<MessageEvent>,
    state: Res<State<GameState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    println!("Go to game !");
                    ev_message.send(MessageEvent(Box::new(ClearMenuMessage)));   
                    ev_message.send(MessageEvent(Box::new(StartGameMessage)));              
                }
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
                    match state.get() {
                        GameState::Disabled => ev_message.send(MessageEvent(Box::new(MainMenuSettingsAudioMessage))),
                        _ => ev_message.send(MessageEvent(Box::new(InGameSettingsAudioMessage)))
                    };
                    //ev_message.send(MessageEvent(Box::new(ClearMenuMessage))); 
                    //ev_message.send(MessageEvent(Box::new(menu_button_action)));       // No Refresh, mais on est pas reconduit au Main Menu si on change le volume ig.
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

