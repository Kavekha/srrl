use bevy::prelude::*;

use super::{components::{MenuButtonAction, OnScreenMenu}, NORMAL_BUTTON, TEXT_COLOR};


pub fn spawn_selection_menu(
    mut commands: Commands,
){
    let button_style = Style {
        width: Val::Px(125.0),
        height: Val::Px(32.5),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 20.0,    //40
        color: TEXT_COLOR,      // AMELIORATION : Mettre dan sle Menu Builder
        ..default()
    };

    let container_menu = commands.spawn(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu,
        )).id();

        let menu_border = commands.spawn(NodeBundle {
            // Cadre du menu en lui-même.
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            //background_color: Color::CRIMSON.into(),
            ..default()
        }).id();

        commands.entity(container_menu).push_children(&[menu_border]);
        
        let action_button = commands.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MenuButtonAction::StartGame
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "START GAME",  //text,
                button_text_style.clone(),
            ));
        }).id();

        commands.entity(menu_border).push_children(&[action_button]);

    }       


