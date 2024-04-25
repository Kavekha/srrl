use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::gamelog::LogEvent, 
};

use super::{
    components::{ UiGameInterface, UiLog, UiLogLine}, 
    UI_LOG_LINE_FONT_SIZE, UI_LOG_LINE_MAX_DURATION_TIME
};


pub fn update_ui_remove_old_lines(
    mut commands: Commands,
    time: Res<Time>,
    mut ui_line_q: Query<(Entity, &mut UiLogLine)>,
){  
    for (line_entity, mut line_timer) in &mut ui_line_q {
        line_timer.tick(time.delta());
        if line_timer.just_finished() {
            commands.entity(line_entity).despawn_recursive();
        }  
    }
}

pub fn update_ui_new_lines(
    mut ev_log: EventReader<LogEvent>,
    mut commands: Commands,  
    interface_query: Query<Entity, With<UiLog>>,    
    assets: Res<GraphicsAssets>,
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
                    align_self: AlignSelf::FlexStart,          
                    ..default()
                }),
            ).insert(UiLogLine(Timer::from_seconds(UI_LOG_LINE_MAX_DURATION_TIME, TimerMode::Once))).id();
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
