use bevy::prelude::*;

pub mod mainmenu;
pub mod recapmenu;
//pub mod victory;
pub mod components;
pub mod ingamemenu;
pub mod menu_builder;
pub mod commons;

use crate::{
    game::despawn_screen, //states::MainMenuState}, 
    globals::{HOVERED_BUTTON, HOVERED_PRESSED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON}};

use self::{commons::CommonsMenuPlugin, components::{OnScreenMenu, SelectedOption}, ingamemenu::InGameMenuPlugin, mainmenu::MainMenuPlugin, recapmenu::RecapMenuPlugin};   //, victory::VictoryPlugin};


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MainMenuPlugin)
            .add_plugins(RecapMenuPlugin)
            .add_plugins(CommonsMenuPlugin)
            .add_plugins(InGameMenuPlugin);
    }
}


pub fn clean_menu(
    mut commands: Commands,
    despawn_onscreenmenu: Query<Entity, With<OnScreenMenu>>,
) {
    println!("Cleaning menu");
    despawn_screen(despawn_onscreenmenu, &mut commands);
}

/// Camera centré sur 0.0,0.0 pour ne pas avoir contenu des menus off screen.
pub fn menu_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
){
    println!("menu camera: ON");
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}


// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}