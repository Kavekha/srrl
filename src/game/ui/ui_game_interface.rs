use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{combat::components::ActionPoints, despawn_component, pieces::components::Health, player::Player, ui::{INTERFACE_HP_CHUNK_HEIGHT, INTERFACE_HP_CHUNK_MAX, INTERFACE_HP_CHUNK_WIDTH}}, 
    globals::INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE
};

use super::components::{UiGameInterface, UiCharacterInfos};


pub fn clear_ui_game_character_infos(
    interface_query: Query<Entity, With<UiCharacterInfos>>,
    commands: &mut Commands,
) {
    println!("DEBUG: Clear interface ui.");
    despawn_component(interface_query, commands);
}

pub fn draw_ui_game_character_infos(
    mut commands: Commands,
    assets: Res<GraphicsAssets>,
    interface_query: Query<Entity, With<UiCharacterInfos>>,
    player_info_query: Query<(Entity, Option<&Name>, &Health), With<Player>>,       //player_info_query: Query<(Entity, &Name, &Health), With<Player>>  // Retrait du Name car au load Save on le perds.
    player_actions_query: Query<(Entity, &ActionPoints), With<Player>>,
) {
    println!("DEBUG: draw ui_game_character_infos");
    clear_ui_game_character_infos(interface_query, &mut commands);

    let mut player_name = "Unkwnown Runner";
    let mut player_health_max = INTERFACE_HP_CHUNK_MAX;
    let mut player_health_current = INTERFACE_HP_CHUNK_MAX;
    if let Ok(player_infos) = player_info_query.get_single() {
        println!("DEBUG : draw interface: Player info OK");
        let (_p_entity, p_name, p_health) = player_infos;   //let (_p_entity, p_name, p_health) = player_infos; // Retrait du name car perdu au save.
        if let Some(name) = p_name {
            player_name = name.as_str();
        }
        //player_name = p_name.as_str();
        player_health_max = p_health.max;
        player_health_current = p_health.current;
        println!("DEBUG: Player health current is {} and max {}", player_health_current, player_health_max);
    } else {
        println!("DEBUG : draw: not player");
    }
    let mut action_points = 0;
    if let Ok(player_action_points) = player_actions_query.get_single() {
        let (_p_entity_action, p_action) = player_action_points;
        action_points = p_action.current;
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
    }).insert(UiGameInterface).insert(UiCharacterInfos).id();  

    let player_action_display = commands.spawn(
        TextBundle::from_section(
            format!("{}",action_points),
            TextStyle { 
                //font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
                font: assets.font.clone(),  
                font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(8.)),            
            ..default()
        }),
    ).id();

    let player_name_tag = commands.spawn((
        TextBundle::from_section(
            player_name,
            TextStyle {
                //font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
                font: assets.font.clone(),  
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
        //UiGameInterface,
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
        chunk_list.push(chunk);
    }
    

    for chunk in chunk_list {
        commands.entity(chunk_container).add_child(chunk);
    }
    
    commands.entity(container).add_child(player_action_display);
    commands.entity(container).add_child(player_name_tag);
    commands.entity(container).add_child(chunk_container);
    
  
}
