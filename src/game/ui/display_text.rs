use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{
        combat::combat_system::components::IsDead, game_generation::character_creation::components::Npc, ui::components:: UiGameInterface}};

use super::components::UiLogLine;

#[derive(Event)]
pub struct TextEvent {
    pub entry: String,
    pub entity: Entity
}

#[derive(Component)]
pub struct UiText;


#[derive(Component)]
pub struct UiTextPosition {
    pub entity_related: Entity
}

const UI_TEXT_MAX_DURATION_TIME: f32 = 2.0;
const UI_TEXT_FONT_SIZE: f32 = 10.0;
const UI_TEXT_COLOR:Color = Color::ANTIQUE_WHITE;
const UI_TEXT_HIGH: f32 = 30.0;
const UI_TEXT_WIDTH: f32 = 180.0;


pub fn update_ui_text_position(
    mut text_style_position_q: Query<(&mut Style, &UiTextPosition)>, 
    camera_q: Query<(&Camera, &GlobalTransform)>, 
    enemies_q: Query<&Transform, (With<Npc>, Without<IsDead>)>,  
){
    let (camera, camera_transform) = camera_q.single();
    let Some(screen_size) = camera.logical_viewport_size() else { return };  

    for (mut style, position) in &mut text_style_position_q {
        let Ok(transform) = enemies_q.get(position.entity_related) else { continue; };
        let Some(mut screen_position) = camera.world_to_viewport(camera_transform, transform.translation)  else { continue };

        //If not in screen, we don't display.
        //if screen_position.x < 0.0 || screen_position.x > screen_size.x || screen_position.y < 0.0 || screen_position.y > screen_size.y { continue};
        if screen_position.x < 0.0 { screen_position.x = 0.0 }
        if screen_position.x > screen_size.x { screen_position.x = screen_size.x }
        if screen_position.y < 0.0 { screen_position.y = 0.0 }
        if screen_position.y > screen_size.x { screen_position.y = screen_size.y }


        let left =screen_position.x - UI_TEXT_WIDTH / 2.0;
        //let right =screen_size.x - screen_position.x;
        let top =screen_position.y - UI_TEXT_HIGH * 2.0; // REMEMBER : world = y goes from bottom to top (++)
        //let bottom = screen_size.y - screen_position.y;        
        let width = UI_TEXT_WIDTH; //INTERFACE_HP_CHUNK_WIDTH * (health.max as f32) / 2.0;
        let height = UI_TEXT_HIGH;
        let grow = 100.0;

        style.left = Val::Px(left);
        style.top = Val::Px(top);
        style.width = Val::Px(width);
        style.height = Val::Px(height);
        style.flex_grow = grow;
    }
}



pub fn draw_and_update_ui_text(
    mut ev_text: EventReader<TextEvent>,
    mut commands: Commands,  
    //interface_query: Query<Entity, With<UiText>>,    
    assets: Res<GraphicsAssets>,
    camera_q: Query<(&Camera, &GlobalTransform)>, 
    enemies_q: Query<&Transform, (With<Npc>, Without<IsDead>)>,  
) {
    let (camera, camera_transform) = camera_q.single();
    let Some(screen_size) = camera.logical_viewport_size() else { return };  

    for event in ev_text.read() {
        info!("Draw&Update: Text recu : {:?}", event.entry);

        let Ok(transform) = enemies_q.get(event.entity) else { continue; };
        let Some(mut screen_position) = camera.world_to_viewport(camera_transform, transform.translation)  else { continue };

        //If not in screen, we don't display.
        //if screen_position.x < 0.0 || screen_position.x > screen_size.x || screen_position.y < 0.0 || screen_position.y > screen_size.y { continue};
        if screen_position.x < 0.0 { screen_position.x = 0.0 }
        if screen_position.x > screen_size.x { screen_position.x = screen_size.x }
        if screen_position.y < 0.0 { screen_position.y = 0.0 }
        if screen_position.y > screen_size.x { screen_position.y = screen_size.y }


        let left =screen_position.x - UI_TEXT_WIDTH;
        //let right =screen_size.x - screen_position.x;
        let top =screen_position.y - UI_TEXT_HIGH; // REMEMBER : world = y goes from bottom to top (++)
        //let bottom = screen_size.y - screen_position.y;        
        let width = UI_TEXT_WIDTH; //INTERFACE_HP_CHUNK_WIDTH * (health.max as f32) / 2.0;
        let height = UI_TEXT_HIGH;
        let grow = 100.0;


        let interface = commands.spawn(NodeBundle {
            style: Style {
                /* 
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                */
                left: Val::Px(left),
                //right: Val::Px(right),
                top: Val::Px(top),
                //bottom: Val::Px(bottom),
                width: Val::Px(width),
                height: Val::Px(height),
                flex_grow: grow,
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::FlexEnd,
                align_items: AlignItems::FlexEnd,            
                flex_direction: FlexDirection::Column,
                bottom: Val::Px(0.),
                border: UiRect::all(Val::Px(2.)), 
                ..default()},                
                border_color: Color::rgba(0.5, 0.5, 0.0, 0.0).into(),
            ..default()
        })
        .insert(UiGameInterface)
        .insert(UiText)
        .insert(UiTextPosition{entity_related: event.entity})
        .insert(UiLogLine(Timer::from_seconds(UI_TEXT_MAX_DURATION_TIME, TimerMode::Once)))
        .id()
        ;

        let log_line = commands.spawn(
            TextBundle::from_section(
                event.entry.to_string(),
                TextStyle { 
                    font: assets.font.clone(),  
                    font_size: UI_TEXT_FONT_SIZE,
                    color: UI_TEXT_COLOR, //Color::ANTIQUE_WHITE,
                },
            )
            .with_style(Style { 
                align_self: AlignSelf::Center,          
                ..default()
            }),
        ).id();
        

        commands.entity(interface).push_children(&[log_line]);
    }
}

