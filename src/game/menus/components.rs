use bevy::prelude::*;

#[derive(Component)]
pub struct OnScreenMenu;

#[derive(Component)]
pub struct OnSettingsMenuScreen;


#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Load,
    Settings,
    SettingsDisplay,
    BackToSettings,
    BackToMainMenu,
    //SettingsSound, 
    QuitConfirm,
    Quit,
    Cancel,
    BackToGame,
    Back,
    DisplayLow,
    DisplayMedium,
    DisplayHigh
}

#[derive(Resource)]
pub struct ResolutionSettings {
    pub low: Vec2,
    pub medium: Vec2,
    pub high: Vec2,
}

#[derive(Component)]
pub struct SelectedOption;


// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    //Low,
    Medium,
    //High,
}



#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameMenuState {
    #[default]
    Disabled,
    MainMenu,
    QuitConfirm,
    Settings,
    SettingDisplay,
    //SettingSound
}