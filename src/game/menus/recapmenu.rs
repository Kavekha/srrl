use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{menus::{components::MenuButtonAction, menu_builder::{spawn_basic_menu, spawn_recap_menu, Menu, MenuView}}, states::MainMenuState}
};

use super::{
    clean_menu, menu_builder::{MenuItem, MenuV2}, menu_camera, OnScreenMenu};


// TODO: Refacto Victory & GameOver en un seul: Recap Screen?

pub struct RecapMenuPlugin;

impl Plugin for RecapMenuPlugin {
    fn build(&self, app: &mut App){
        app
            .add_event::<MenuEvent>()
            //.add_systems(OnEnter(MainMenuState::RecapMenu), display_gameover_screen) //TEST  
            //.add_systems(OnEnter(MainMenuState::RecapMenu), enter_go_menu)
            //.add_systems(OnEnter(MainMenuState::RecapMenu), enter_recap_menu)
            .add_systems(Update, menu_event_reader.run_if(on_event::<MenuEvent>()));
            //.add_systems(OnEnter(MainMenuState::RecapMenu), menu_camera)         
            //.add_systems(OnExit(MainMenuState::RecapMenu), clean_menu); 

   
        
    }
}

#[derive(Event)]
pub struct MenuEvent{
    pub id: String,
    pub header: String,
    pub description: String,
    pub menu_type: MenuType
}

pub enum MenuType {
    RECAPMENU,
    MAINMENU
}

fn menu_event_reader(
    mut commands: Commands,
    mut ev_menu: EventReader<MenuEvent>,
    graph_assets: Res<GraphicsAssets>,
) {
    for event in ev_menu.read() {
        let menu = MenuV2::new(
            &event.id,
            vec![
                    MenuItem::header(&event.header),
                    MenuItem::description(&event.description),
                    MenuItem::action(MenuButtonAction::Play, "Retry"),
                    MenuItem::action(MenuButtonAction::BackToMainMenu, "MainMenu")
            ]
        );
        println!("Je suis dans Menu Event Reader avec pour header: {}.", event.header);
        spawn_recap_menu(&mut commands, graph_assets, menu);
        break;      // Degueu, mais seul le premier m'interesse et c peu probable que j'en ai d'autres.
    }    
}

pub fn enter_recap_menu(    
    mut commands: Commands,
    graph_assets: Res<GraphicsAssets>,
){
    println!("Je ne suis plus sensé fonctionner");
    let menu = MenuV2::new(
        "recap_menu",
        vec![
                MenuItem::header("You died."),
                MenuItem::description("A ghoul has eaten you."),
                MenuItem::action(MenuButtonAction::Play, "Retry"),
                MenuItem::action(MenuButtonAction::BackToMainMenu, "MainMenu")
        ]
    );
    spawn_recap_menu(&mut commands, graph_assets, menu)
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

