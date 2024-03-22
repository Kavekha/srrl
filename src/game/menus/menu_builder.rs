
use bevy::prelude::*;   //, app::AppExit};

use crate::{
    game::menus::components::OnScreenMenu, globals::{NORMAL_BUTTON, TEXT_COLOR},
    engine::asset_loaders::GraphicsAssets
};

use super::components::MenuButtonAction;


pub struct MenuView{
    pub action: MenuButtonAction,
    pub text: String,
}
impl MenuView {
    pub fn new(action: MenuButtonAction, text:String
    ) -> MenuView {
        let menu = MenuView {action: action, text:text};
        menu
    }
}


pub struct Menu{
    pub pages: Vec<MenuView>
}
impl Menu {
    pub fn new() -> Menu {
        let menu = Menu{pages:Vec::new()};
        menu
    }
}

//MenuBuilder v2
pub struct Action {pub action: MenuButtonAction, pub text:String}
pub struct Header {pub text: String}
pub struct Description {pub text: String}

/*
pub enum MenuItem{
    Action{action: MenuButtonAction, text:String},
    Header{text:String},
    Description{text:String}
} */
pub enum MenuItem{
    Action(Action),
    Header(Header),
    Description(Description)
}

impl MenuItem{
    pub fn action(action:MenuButtonAction, text:&str
    ) -> MenuItem {
        MenuItem::Action(Action{action: action, text:text.to_string()})
    }
    pub fn header(text:&str
    ) -> MenuItem {
        MenuItem::Header(Header{text:text.to_string()})
    }
    pub fn description(text:&str
    ) -> MenuItem {
        MenuItem::Description(Description{text:text.to_string()})
    }
}

pub struct MenuV2{
    pub id: String,
    pub entries: Vec<MenuItem>
}
impl MenuV2{
    pub fn new(id: &str, entries: Vec<MenuItem>) -> MenuV2 {
        let menu = MenuV2{
            id: id.to_string(),
            entries: entries
        };
        menu
    } 
}

pub fn read_menu() -> MenuV2{
    let new_menu = MenuV2::new(
        "recap_menu",
        vec![
                MenuItem::header("You died."),
                MenuItem::description("A ghoul has eaten you."),
                MenuItem::action(MenuButtonAction::Play, "Retry"),
                MenuItem::action(MenuButtonAction::BackToMainMenu, "MainMenu")
        ]
    );
    new_menu
}


// Them 2 : Recap Menu
pub fn spawn_recap_menu(
commands: &mut Commands,
graph_assets: Res<GraphicsAssets>,
menu: MenuV2
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

// Theme 1 : Classic Menu

pub fn spawn_basic_menu(commands: &mut Commands, new_menu: Menu) {
    println!("In Game Menu");
    //let new_menu = Menu::new();

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

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu,   //OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    //background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for page in new_menu.pages 
                    /* 
                    for (action, text) in [
                                                (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] */
                    {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                page.action,    //action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    page.text,  //text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}
