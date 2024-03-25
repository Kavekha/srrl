use bevy::prelude::*;

pub mod components;
pub mod menu_builder;
pub mod commons;

use crate::{
    engine::asset_loaders::GraphicsAssets,
    game::{despawn_screen, menus::menu_builder::spawn_recap_menu}, //states::MainMenuState}, 
    globals::{HOVERED_BUTTON, HOVERED_PRESSED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON}};

use self::{
    commons::CommonsMenuPlugin, components::{OnScreenMenu, SelectedOption}, 
    menu_builder::Menu
};


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CommonsMenuPlugin)
            //Transfert vers le menu
            .add_event::<MenuEvent>()
            .add_systems(Update, menu_event_reader.run_if(on_event::<MenuEvent>()));        
    }
}

// TODO : Refaire, car pas souple du tout. Ca construit le Menu par procuration, car on recoit un Event depuis World. C'est très moche.
#[derive(Event)]
pub struct MenuEvent{
    pub menu: Menu,
    pub menu_type: MenuType     //Type pour savoir quel menu on créé? Au cas où pôur le moment.
}

pub enum MenuType {
    RECAPMENU,
    MAINMENU,
    SETTINGS,
    DISPLAY,
    QUIT
}

fn menu_event_reader(
    mut commands: Commands,
    mut ev_menu: EventReader<MenuEvent>,
    graph_assets: Res<GraphicsAssets>,
) {
    for event in ev_menu.read() {
        //println!("Je suis dans Menu Event Reader avec pour type: {:?}.", event.menu_type);
        println!("Menu reçu et envoyé.");
        let menu = &event.menu;
        spawn_recap_menu(&mut commands, graph_assets, menu);
        break;      // Degueu, mais seul le premier m'interesse et c peu probable que j'en ai d'autres.
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
