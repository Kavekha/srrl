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

const INTERFACE_HP_CHUNK_MAX: u32 = 20;




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



pub fn draw_interface(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interface_query: Query<Entity, With<InterfaceGame>>,
    player_info_query: Query<(Entity, &Name, &Health), With<Player>>
) {
    clear_interface(&mut commands, interface_query);

    // Interface container.
    let container = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).id();  

    /* 
    let name_container = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(10.0),
            height: Val::Percent(10.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).id();  
    */

    let mut player_name = "Unkwnown Runner";
    let mut player_health_max = INTERFACE_HP_CHUNK_MAX;
    let mut player_health_current = INTERFACE_HP_CHUNK_MAX;
    if let Ok(player_infos) = player_info_query.get_single() {
        let (p_entity, p_name, p_health) = player_infos;
        player_name = p_name.as_str();
        player_health_max = p_health.max;
        player_health_current = p_health.current;
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

    let mut chunk_list:Vec<Entity> = Vec::new();
    for i in 1..=player_health_max {
        println!("Chunk {:?}", i);
        let mut border_color = Color::rgb(0.5, 0.0, 0.0);
        let mut background_color = Color::rgb(0.9, 0.0, 0.0 );
        if i > player_health_current {
            border_color = Color::rgb(0.1, 0.1, 0.1);
            background_color = Color::rgba(0.0, 0.0, 0.0, 1.0 );
        }

        let chunk = commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(8.0),
                height: Val::Px(16.0),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::FlexStart,
                flex_grow: 8.0,
                bottom: Val::Px(10.),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: border_color.into(),  // Color::rgb(0.5, 0.0, 0.0).into(), //Color::RED.into(),
            //background_color: Color::rgb(1.0, 0.0, 0.3).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_grow: 8.0,
                    //width: Val::Px(6.0),
                    //height: Val::Px(14.0),
                    ..default()
                },
                background_color: background_color.into(), //Color::rgb(0.9, 0.0, 0.0 ).into(),
                ..default()
            });  
        }).id();
        commands.entity(chunk).insert(InterfaceGame);
        chunk_list.push(chunk);
    }
    

    commands.entity(container).add_child(player_name_tag);
    for chunk in chunk_list {
        commands.entity(container).add_child(chunk);
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