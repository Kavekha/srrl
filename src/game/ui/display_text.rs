use bevy::{prelude::*, text::{BreakLineOn, Text2dBounds}};

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{
        combat::combat_system::components::IsDead, pieces::components::Npc, 
        ui::components:: UiGameInterface}, globals::STANDARD_TILE_SIZE};

use super::{components::UiLogLine, UI_LOG_LINE_FONT_SIZE};

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

const UI_TEXT_MAX_DURATION_TIME: f32 = 12.0;




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


        let left =screen_position.x - ((STANDARD_TILE_SIZE / 2) as f32);
        //let right =screen_size.x - screen_position.x;
        let top =screen_position.y - ((STANDARD_TILE_SIZE / 2) as f32); // REMEMBER : world = y goes from bottom to top (++)
        //let bottom = screen_size.y - screen_position.y;        
        let width = 80.0; //INTERFACE_HP_CHUNK_WIDTH * (health.max as f32) / 2.0;
        let height = 80.0;
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


        let left =screen_position.x - ((STANDARD_TILE_SIZE / 2) as f32);
        //let right =screen_size.x - screen_position.x;
        let top =screen_position.y - ((STANDARD_TILE_SIZE / 2) as f32); // REMEMBER : world = y goes from bottom to top (++)
        //let bottom = screen_size.y - screen_position.y;        
        let width = 80.0; //INTERFACE_HP_CHUNK_WIDTH * (health.max as f32) / 2.0;
        let height = 80.0;
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
                ..default()},
            ..default()
        }).insert(UiGameInterface).insert(UiText).insert(UiTextPosition{entity_related: event.entity}).id()
        ;
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
        ).insert(UiLogLine(Timer::from_seconds(UI_TEXT_MAX_DURATION_TIME, TimerMode::Once))).id();
        commands.entity(interface).push_children(&[log_line]);
    }
}



pub fn draw_and_update_ui_text_v1(
    mut ev_text: EventReader<TextEvent>,
    mut commands: Commands,  
    interface_query: Query<Entity, With<UiText>>,    
    assets: Res<GraphicsAssets>,
) {
    for event in ev_text.read() {
        info!("Draw&Update: Text recu : {:?}", event.entry);

        let interface = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::FlexEnd,
                align_items: AlignItems::FlexEnd,            
                flex_direction: FlexDirection::Column,
                bottom: Val::Px(0.),
                ..default()},
            ..default()
        }).insert(UiGameInterface).insert(UiText).id()
        ;
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
        ).insert(UiLogLine(Timer::from_seconds(UI_TEXT_MAX_DURATION_TIME, TimerMode::Once))).id();
        commands.entity(interface).push_children(&[log_line]);
    }
}


pub fn update_ui_text_v1(
    mut ev_text: EventReader<TextEvent>,
    mut commands: Commands,  
    interface_query: Query<Entity, With<UiText>>,    
    assets: Res<GraphicsAssets>,
) {
    if let Ok(ui_container) = interface_query.get_single() {
        let mut lines = Vec::new();
        for event in ev_text.read() {  
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
            ).insert(UiLogLine(Timer::from_seconds(UI_TEXT_MAX_DURATION_TIME, TimerMode::Once))).id();
            lines.push(log_line); 
        }  
        for line in lines {
            commands.entity(ui_container).push_children(&[line]);
        }
    }
    
}


pub fn draw_text_ui(
    mut commands: Commands,    
){
    // Interface container.
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd,
            align_content: AlignContent::FlexEnd,
            align_items: AlignItems::FlexEnd,            
            flex_direction: FlexDirection::Column,
            bottom: Val::Px(0.),
            ..default()},
        ..default()
    }).insert(UiGameInterface).insert(UiText);
}


//v1
pub fn display_text_box(
    mut ev_text_box: EventReader<TextEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>

){
    for event in ev_text_box.read() {
        info!("Display event received for {:?}", event.entry);
        let font = asset_server.load("fonts/PressStart2P-vaV7.ttf");
        let box_size = Vec2::new(300.0, 200.0);
        //let box_position = Vec2::new(0.0, -250.0);
        let box_position = Vec2::new(0.0, 0.0);

        let slightly_smaller_text_style = TextStyle {
            font,
            font_size: 50.0,
            color: Color::WHITE,
        };

        commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                        ..default()
                    },
                    transform: Transform::from_translation(box_position.extend(0.0)),
                    ..default()
                })
                .insert(UiGameInterface)
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                event.entry.clone(),
                                slightly_smaller_text_style.clone(),
                            )],
                            justify: JustifyText::Left,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        },
                        text_2d_bounds: Text2dBounds {
                            // Wrap text in the rectangle
                            size: box_size,
                        },
                        // ensure the text is drawn on top of the box
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                });
    }
    
}
