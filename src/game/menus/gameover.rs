use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, game::{menus::{components::MenuButtonAction, menu_builder::{spawn_basic_menu, Menu, MenuView}}, states::MainMenuState}
};

use super::{
    clean_menu, menu_camera, OnScreenMenu};


// TODO: Refacto Victory & GameOver en un seul: Recap Screen?

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(MainMenuState::RecapMenu), enter_go_menu)
            .add_systems(OnEnter(MainMenuState::RecapMenu), menu_camera)         
            .add_systems(OnExit(MainMenuState::RecapMenu), clean_menu);    
    }
}

pub fn enter_go_menu(mut commands: Commands) {
    println!("Entering GameOver menu.");
    let mut menu = Menu::new();
    for (action, text) in [                            
            (MenuButtonAction::Play, "Retry"),
            (MenuButtonAction::BackToMainMenu, "MainMenu"),
        ] {
            let page = MenuView::new(action, text.to_string());
            menu.pages.push(page);
    }
    spawn_basic_menu(&mut commands, menu)
}


fn _display_gameover_screen(
    mut commands: Commands,
    graph_assets: Res<GraphicsAssets>
) {

    commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                OnScreenMenu
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "YOU DIED.",
                    TextStyle {
                        font: graph_assets.font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "A ghoul has eaten you.",
                    TextStyle {
                        font: graph_assets.font.clone(),
                        font_size: 20.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ));
            });

}

