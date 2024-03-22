use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, game::{menus::{components::MenuButtonAction, menu_builder::{spawn_basic_menu, Menu, MenuView}}, states::MainMenuState}, globals::{NORMAL_BUTTON, TEXT_COLOR}
};

use super::{
    clean_menu, menu_builder::{read_menu, spawn_recap_menu, Description, MenuItem, MenuV2}, menu_camera, OnScreenMenu};


// TODO: Refacto Victory & GameOver en un seul: Recap Screen?

pub struct RecapMenuPlugin;

impl Plugin for RecapMenuPlugin {
    fn build(&self, app: &mut App){
        app
            .add_event::<MenuEvent>()
            //.add_systems(OnEnter(MainMenuState::RecapMenu), display_gameover_screen) //TEST  
            //.add_systems(OnEnter(MainMenuState::RecapMenu), enter_go_menu)
            //.add_systems(OnEnter(MainMenuState::RecapMenu), enter_recap_menu)
            .add_systems(Update, menu_event_reader.run_if(on_event::<MenuEvent>()))
            .add_systems(OnEnter(MainMenuState::RecapMenu), menu_camera)         
            .add_systems(OnExit(MainMenuState::RecapMenu), clean_menu); 

   
        
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
    RECAP_MENU
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
        spawn_recap_menu(&mut commands, graph_assets, menu);
        break;      // Degueu, mais seul le premier m'interesse et c peu probable que j'en ai d'autres.
    }    
}

pub fn enter_recap_menu(    
    mut commands: Commands,
    graph_assets: Res<GraphicsAssets>,
){
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


fn display_gameover_screen(
    mut commands: Commands,
    graph_assets: Res<GraphicsAssets>
) {
    let button_style = Style {
        width: Val::Px(100.0),
        height: Val::Px(32.5),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 20.0,
        color: TEXT_COLOR,
        ..default()
    };
    
    let menu = read_menu();

    commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                OnScreenMenu
            ))
            .with_children(|parent| {
                for item in menu.entries {
                    match item {
                        MenuItem::Header(header) => {
                            parent.spawn(TextBundle::from_section(
                                header.text,   //"YOU DIED.",
                                TextStyle {
                                    font: graph_assets.font.clone(),
                                    font_size: 30.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                },
                            ));
                        },
                        MenuItem::Description(description) => {
                            parent.spawn(TextBundle::from_section(
                                description.text,   //"YOU DIED.",
                                TextStyle {
                                        font: graph_assets.font.clone(),
                                        font_size: 15.0,
                                        color: Color::rgb(1.0, 1.0, 1.0),
                                    },
                                ));
                            },
                            MenuItem::Action(action) => {
                                parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    action.action,    //action,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        action.text,  //text,
                                        button_text_style.clone(),
                                    ));
                                });
                            }
                        
                        _ => println!("MenuItem non géré")
                    };
                }

                    {
                        
                    }
            });
}
