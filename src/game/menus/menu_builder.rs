
use bevy::prelude::*;

use crate::{
    game::menus::components::OnScreenMenu, globals::{NORMAL_BUTTON, TEXT_COLOR},
    engine::asset_loaders::GraphicsAssets
};

use super::components::MenuButtonAction;


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

pub struct Menu{
    pub id: String,
    pub entries: Vec<MenuItem>
}
impl Menu{
    pub fn new(id: &str, entries: Vec<MenuItem>) -> Menu {
        let menu = Menu{
            id: id.to_string(),
            entries: entries
        };
        menu
    } 
    pub fn add(&mut self, menu_item: MenuItem){
        self.entries.push(menu_item);
    }
}


// Them 2 : Recap Menu
pub fn spawn_recap_menu(
commands: &mut Commands,
graph_assets: Res<GraphicsAssets>,
menu: &Menu
) {
    println!("Spawning recap menu.");
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
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu
        ))
        .with_children(|parent| {
            for item in &menu.entries {
                match item {
                    MenuItem::Header(header) => {
                        parent.spawn(TextBundle::from_section(
                            header.text.clone(),   //"YOU DIED.",
                            TextStyle {
                                font: graph_assets.font.clone(),
                                font_size: 30.0,
                                color: Color::rgb(1.0, 1.0, 1.0),
                            },
                        ));
                    },
                    MenuItem::Description(description) => {
                        parent.spawn(TextBundle::from_section(
                            description.text.clone(),   //"YOU DIED.",
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
                                action.action.clone(),    //action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    action.text.clone(),  //text,
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

//Not compatible with MenuV2
pub fn _spawn_basic_menu(commands: &mut Commands, new_menu: Menu) {
    println!("In Game Menu");
    //let new_menu = Menu::new();

    /* 
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
        */
}
