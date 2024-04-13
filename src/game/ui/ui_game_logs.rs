use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{despawn_component, gamelog::{Gamelog, LogEvent}}, 
};

use super::{components::{ UiGameInterface, UiLog, UiLogLine}, INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE, UI_LOG_LINES_MAX, UI_LOG_LINE_FONT_SIZE, UI_LOG_LINE_MAX_DURATION_TIME, UI_LOG_OLDER_LINE_DURATION_TIME};


pub fn update_ui_remove_old_lines(
    mut commands: Commands,
    time: Res<Time>,
    mut ui_line_q: Query<Entity, With<UiLogLine>>,
){  
    for line in &mut ui_line_q {
        if time.elapsed_seconds() >= 10.0 {
            commands.entity(line).despawn_recursive();
            info!("Removed line because too old.");
        }
        //info!("update ui finished.");
    }
}
pub fn update_ui_new_lines(
    mut ev_log: EventReader<LogEvent>,
    mut commands: Commands,  
    interface_query: Query<Entity, With<UiLog>>,    
    assets: Res<GraphicsAssets>,
    time: Res<Time>,
) {
    if let Ok(ui_container) = interface_query.get_single() {
        let mut lines = Vec::new();
        for event in ev_log.read() {  
            let log_line = commands.spawn(
                TextBundle::from_section(
                    event.entry.to_string(),
                    TextStyle { 
                        font: assets.font.clone(),  
                        font_size: UI_LOG_LINE_FONT_SIZE,
                        color: Color::YELLOW,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(8.)),  
                    align_self: AlignSelf::FlexStart,          
                    ..default()
                }),
            ).insert(UiLogLine).id();
            lines.push(log_line); 
        }  
        for line in lines {
            commands.entity(ui_container).push_children(&[line]);
        }
    }
    
}

pub fn draw_log_ui(
    mut commands: Commands,    
){
    // Interface container.
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
            align_items: AlignItems::FlexStart,            
            flex_direction: FlexDirection::Column,
            bottom: Val::Px(0.),
            ..default()},
        ..default()
    }).insert(UiGameInterface).insert(UiLog);

}


fn clear_ui_log(
    commands: &mut Commands,    
    interface_query: Query<Entity, With<UiLog>>,
) {
    //println!("DEBUG: Clear Logs UI");
    despawn_component(interface_query, commands);
}

pub fn draw_log_ui_old(
    mut commands: Commands,
    game_log: Res<Gamelog>,
    assets: Res<GraphicsAssets>,
    interface_query: Query<Entity, With<UiLog>>,
){
    // Texte a afficher. Devrait etre dans une fonction séparée? 
    /*/
    let mut logs = "".to_string();
    for log in game_log.entries.iter().rev().take(4).rev() {
        logs = format!("{}{}\n", logs, log.clone());
        println!("LOG:Added to Log: {}", log.clone());
    }*/
    let logs = game_log.get_last_entries_as_string(4);

    clear_ui_log(&mut commands, interface_query);
    // Interface container.
    let container = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            bottom: Val::Px(0.),
            ..default()},
        ..default()
    }).insert(UiGameInterface).insert(UiLog).id();


        let log_line = commands.spawn(
            TextBundle::from_section(
                logs.clone(),
                TextStyle { 
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
        commands.entity(container).push_children(&[log_line]);
}