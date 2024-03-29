use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{despawn_component, gamelog::Gamelog}, 
    globals::INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE
};

use super::components::{UiGameInterface, UiLog};


pub fn clear_ui_log(
    commands: &mut Commands,    
    interface_query: Query<Entity, With<UiLog>>,
) {
    println!("DEBUG: Clear Logs UI");
    despawn_component(interface_query, commands);
}

pub fn draw_log_ui(
    mut commands: Commands,
    game_log: Res<Gamelog>,
    assets: Res<GraphicsAssets>,
    interface_query: Query<Entity, With<UiLog>>,
){
    // Texte a afficher. Devrait etre dans une fonction séparée?
    let mut logs = "".to_string();
    for log in game_log.entries.iter().rev().take(4).rev() {
        logs = format!("{}{}\n", logs, log.clone());
        println!("LOG:Added to Log: {}", log.clone());
    }
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