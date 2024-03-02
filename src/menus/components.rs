use bevy::prelude::*;

#[derive(Component)]
pub struct OnScreenMenu;

#[derive(Component)]
pub struct OnSettingsMenuScreen;


#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Load,
    SettingsDisplay,
    BackToSettings,
    BackToMainMenu,
    SettingsSound, 
    Quit
}

#[derive(Component)]
pub struct SelectedOption;


// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}