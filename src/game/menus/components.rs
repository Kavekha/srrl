use bevy:: prelude::*;
use crate::engine::audios::AudioType;

#[derive(Component)]
pub struct OnScreenMenu;

#[derive(Component)]
pub struct OnSettingsMenuScreen;


#[derive(Component, Clone)]
pub enum MenuButtonAction {
    Play,
    Load,
    MainMenuSettings,
    BackToMainMenu,
    MainMenuSettingsDisplay,
    MainMenuSettingsAudio,
    SettingsAudioChange{modify_volume_by:f32, audio_type:AudioType}, //, original_volume: Volume},
    DisplayLow,
    DisplayMedium,
    DisplayHigh,
    QuitConfirm,
    Quit,
    Close,
    InGameMenuSettings,
    InGameMenuQuit,
    BackToInGameMenu,
    InGameMenuDisplay,
    InGameMenuAudio,
}

#[derive(Resource)]
pub struct ResolutionSettings {
    pub low: Vec2,
    pub medium: Vec2,
    pub high: Vec2,
}

#[derive(Component)]
pub struct SelectedOption;
