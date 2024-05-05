use bevy::prelude::*;

use crate::menu_builders::menus::{components::SelectedOption, NORMAL_BUTTON};

use super::components::{KindProposition, PlayerCreation};


pub fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        },
    ));
}


/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
pub fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

pub fn item_rect_double(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_column: GridPlacement::span(8),                
                grid_row: GridPlacement::span(5),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

pub fn item_rect_triple(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_column: GridPlacement::span(8),
                grid_row: GridPlacement::span(10),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

pub fn item_rect_metatype_selection_title(builder: &mut ChildBuilder, color: Color, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                padding: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {            
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Choose your Meta-type:",
                    TextStyle {
                        font: font.clone(),
                        font_size: 10.0,
                        color: Color::WHITE,                                                                    
                        ..default()
                    }
                ));
            });               
        });
}


pub fn item_rect_metatype_selection_choice(builder: &mut ChildBuilder, color: Color, font: Handle<Font>, name: String) {
    let button_style = Style {
        //width: Val::Px(125.0),
        //height: Val::Px(32.5),
        //margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                grid_row: GridPlacement::span(1),
                padding: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {            
            /* 
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            });*/
            builder.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                //MenuButtonAction::StartGame                
                KindProposition { kind : name.clone() }
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    name.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 15.0,
                        color: Color::WHITE,                                            
                        ..default()
                    }
                ));
            });               
        });
}


pub fn selecting_kind(
    interaction_q: Query<(&Interaction, &KindProposition, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_q: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,       // Ici on récupère l'element déjà selectionné s'il existe.
    mut commands: Commands,
    mut player_creation: ResMut<PlayerCreation>,
) {
    for (interaction, kind_proposal, entity) in &interaction_q {
        if *interaction == Interaction::Pressed && player_creation.kind != kind_proposal.kind {
            //Si je presse un bouton qui concerne un Kind different de celui que j'ia deja selectionné =>
            if !selected_q.is_empty() {            
                let (previous_entity, mut previous_bg) = selected_q.single_mut();
                previous_bg.0 = NORMAL_BUTTON.into();
                commands.entity(previous_entity).remove::<SelectedOption>();
            }
            commands.entity(entity).insert(SelectedOption);
            player_creation.kind = kind_proposal.kind.clone();
        }        
    }
}
