
use bevy::prelude::*;   //, app::AppExit};

use crate::{game::menus::components::OnScreenMenu, globals::{NORMAL_BUTTON, TEXT_COLOR}};

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
