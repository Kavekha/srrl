use bevy::prelude::*;

use crate::{game::{pieces::components::Health, player::Player}, globals::INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE};

const INTERFACE_HP_CHUNK_HEIGHT: f32 = 16.;
const INTERFACE_HP_CHUNK_WIDTH: f32 = 8.;

const INTERFACE_HP_CHUNK_MAX: u32 = 20;




#[derive(Component)]
pub struct InterfaceGame;



pub fn draw_interface(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interface_query: Query<Entity, With<InterfaceGame>>,
    player_info_query: Query<(Entity, &Name, &Health), With<Player>>
) {
    clear_interface(&mut commands, interface_query);

    let mut player_name = "Unkwnown Runner";
    let mut player_health_max = INTERFACE_HP_CHUNK_MAX;
    let mut player_health_current = INTERFACE_HP_CHUNK_MAX;
    if let Ok(player_infos) = player_info_query.get_single() {
        let (_p_entity, p_name, p_health) = player_infos;
        player_name = p_name.as_str();
        player_health_max = p_health.max;
        player_health_current = p_health.current;
    }   


    // Interface container.
    let container = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).insert(InterfaceGame).id();  

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
            margin: UiRect::all(Val::Px(8.)),            
            ..default()
        }),
        // Because this is a distinct label widget and
        // not button/list item text, this is necessary
        // for accessibility to treat the text accordingly.
        Label,
        //InterfaceGame,
    )).id();


    let chunk_container = commands.spawn(NodeBundle {
        style: Style {
            ..default()
        },
        ..default()
    }).id();  
 

    let mut chunk_list:Vec<Entity> = Vec::new();
    for i in 1..=player_health_max {
        let mut border_color = Color::rgb(0.5, 0.0, 0.0);
        let mut background_color = Color::rgb(0.9, 0.0, 0.0 );
        if i > player_health_current {
            border_color = Color::rgb(0.1, 0.1, 0.1);
            background_color = Color::rgba(0.0, 0.0, 0.0, 1.0 );
        }

        let chunk = commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(INTERFACE_HP_CHUNK_WIDTH),//(8.0),
                height: Val::Px(INTERFACE_HP_CHUNK_HEIGHT), //(16.0),
                margin: UiRect::all(Val::Px(1.)),   
                flex_grow: 8.0,
                bottom: Val::Px(8.),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: border_color.into(), 
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_grow: 8.0,
                    ..default()
                },
                background_color: background_color.into(),
                ..default()
            });  
        }).id();
        //commands.entity(chunk).insert(InterfaceGame);
        chunk_list.push(chunk);
    }
    

    for chunk in chunk_list {
        commands.entity(chunk_container).add_child(chunk);
    }
    commands.entity(container).add_child(player_name_tag);
    commands.entity(container).add_child(chunk_container);
    
  
}

fn clear_interface(
    commands: &mut Commands,
    interface_query: Query<Entity, With<InterfaceGame>>
) {
    for entity in interface_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}