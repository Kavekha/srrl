use bevy::prelude::*;

#[derive(Component)]
pub struct OnScreenMenu;

#[derive(Component)]
pub struct MainMenuClickable {
    pub size: Vec2,
    pub id: MainMenuOptions
}

#[derive(Component)]
pub struct NineSlice;


#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub enum MainMenuOptions {
    StartGame,
    LoadGame,
    Quit
}

#[derive(Resource)]
pub struct MainMenuSelection {
    pub selected: MainMenuOptions
}

# [derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);