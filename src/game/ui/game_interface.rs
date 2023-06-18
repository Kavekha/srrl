use bevy::{prelude::*, a11y::{AccessibilityNode, accesskit::{NodeBuilder, Role}}};

use crate::{game::{pieces::components::Health, player::Player}, globals::INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE};


const INTERFACE_GLOBAL_HEIGHT: f32 = 40.;
const INTERFACE_GLOBAL_WIDTH: f32 = 96.;
const INTERFACE_NAME_HEIGHT: f32 = 32.;
const INTERFACE_NAME_WIDTH: f32 = 20.;

const INTERFACE_HP_BLOCK_HEIGHT: f32 = 32.;
const INTERFACE_HP_BLOCK_WIDTH: f32 = 300.;
const INTERFACE_HP_CHUNK_MARGIN: f32 = 2.;
const INTERFACE_HP_CHUNK_HEIGHT: f32 = 24.;
const INTERFACE_HP_CHUNK_WIDTH: f32 = 16.;

const INTERFACE_HP_CHUNK_MAX: i32 = 20;




#[derive(Component)]
pub struct InterfaceGame;

#[derive(Component)]
pub struct InterfacePlayerName;

#[derive(Component)]
pub struct InterfaceHealthChunk;


#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

/* 
pub fn draw_interface_test(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        ),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
                                    }
                                });
                        });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(210.),
                        bottom: Val::Px(10.),
                        border: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    border_color: Color::GREEN.into(),
                    background_color: Color::rgb(0.4, 0.4, 1.).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::rgb(0.8, 0.8, 1.).into(),
                        ..default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(1.0, 0.0, 0.).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    // Take the size of the parent node.
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(20.),
                                    bottom: Val::Px(20.),
                                    ..default()
                                },
                                background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(40.),
                                    bottom: Val::Px(40.),
                                    ..default()
                                },
                                background_color: Color::rgb(1.0, 0.5, 0.5).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(60.),
                                    bottom: Val::Px(60.),
                                    ..default()
                                },
                                background_color: Color::rgb(1.0, 0.7, 0.7).into(),
                                ..default()
                            });
                            // alpha test
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(80.),
                                    bottom: Val::Px(80.),
                                    ..default()
                                },
                                background_color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                ..default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    // A `NodeBundle` is used to display the logo the image as an `ImageBundle` can't automatically
                    // size itself with a child node present.
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(500.0),
                                    height: Val::Px(125.0),
                                    margin: UiRect::top(Val::VMin(5.)),
                                    ..default()
                                },
                                // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UiImage::new(asset_server.load("branding/bevy_logo_dark_big.png")),
                        ))
                        .with_children(|parent| {
                            // alt text
                            // This UI node takes up no space in the layout and the `Text` component is used by the accessiblity module
                            // and is not rendered.
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::None,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                Text::from_section("Bevy logo", TextStyle::default()),
                            ));
                        });
                });
        });
}
*/


pub fn draw_interface(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interface_query: Query<Entity, With<InterfaceGame>>,
    player_info_query: Query<(Entity, &Name, &Health), With<Player>>
) {
    clear_interface(&mut commands, interface_query);

    let container = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).id();  

    let mut player_name = "Unkwnown Runner";
    if let Ok(player_infos) = player_info_query.get_single() {
        let (p_entity, p_name, p_health) = player_infos;
        player_name = p_name.as_str();
    }   


    let player_name_tag = commands.spawn((
        TextBundle::from_section(
            player_name,
            TextStyle {
                font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
                font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        }),
        // Because this is a distinct label widget and
        // not button/list item text, this is necessary
        // for accessibility to treat the text accordingly.
        Label,
        InterfaceGame,
    )).id();

    commands.entity(container).add_child(player_name_tag);

    for i in 0..INTERFACE_HP_CHUNK_MAX {
        println!("i in interface hp chunk max is : {:?}", i);
    }

    /* 
    let container = commands.spawn((
        InterfaceGame,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                border: UiRect {bottom:Val::Px(0.), left: Val::Px(0.), right: Val::Px(0.), top: Val::Px(0.)},
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Px(INTERFACE_GLOBAL_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        }))
        .id();
        
    for i in 0..INTERFACE_HP_CHUNK_MAX {
        let mut margin = UiRect::all(Val::Px(INTERFACE_HP_CHUNK_MARGIN));
        margin.bottom = Val::Px(INTERFACE_HP_CHUNK_MARGIN);
        let chunk = helpers::get_chunk(
            &mut commands,
            Size::new(Val::Px(INTERFACE_HP_CHUNK_WIDTH, Val::Px(INTERFACE_HP_CHUNK_HEIGHT)),
            margin,
            &assets.textures["hp_chunk"],
        ));
    }
    */
}

fn clear_interface(
    commands: &mut Commands,
    interface_query: Query<Entity, With<InterfaceGame>>
) {
    for entity in interface_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}