use bevy::prelude::*;

use crate::ecs_elements::components::MainMenuOptions;

#[derive(Resource)]
pub struct MainMenuSelection {
    pub selected: MainMenuOptions
}