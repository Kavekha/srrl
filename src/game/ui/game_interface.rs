use bevy::prelude::*;

use crate::{
    game::{pieces::components::{Health, Monster, Occupier}, player::{Player, Cursor}, combat::{components::ActionPoints, event_systems::{get_ap_cost, ActionInfos}}, tileboard::components::BoardPosition},
    globals::{INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE, TILE_WIDTH_HALF, TILE_HEIGHT_HALF, CHAR_SIZE}, render::components::GameCursorRender, map_builders::map::Map
};

use super::components::{InterfaceGame, UiEnemyHp, UiActionPointsOnCursor};

const INTERFACE_HP_CHUNK_HEIGHT: f32 = 16.;
const INTERFACE_HP_CHUNK_WIDTH: f32 = 8.;

const INTERFACE_HP_CHUNK_MAX: u32 = 20;


pub fn clear_interface(
    commands: &mut Commands,
    interface_query: Query<Entity, With<InterfaceGame>>,
) {
    for entity in interface_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_enemy_hp_ui(
    commands: &mut Commands,    
    interface_query: Query<Entity, With<UiEnemyHp>>,
) {
    for entity in interface_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_action_points_cursor_ui(
    commands: &mut Commands,    
    interface_query: Query<Entity, With<UiActionPointsOnCursor>>,
) {
    for entity in interface_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn display_action_points_on_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cursor: Res<Cursor>,
    camera_q: Query<(&Camera, &GlobalTransform)>, 
    query_game_cursor: Query<&mut Transform, With<GameCursorRender>>,
    interface_query: Query<Entity, With<UiActionPointsOnCursor>>,
    player_q: Query<Entity, With<Player>>,
    query_character: Query<(&ActionPoints, &BoardPosition)>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
    action_infos: Res<ActionInfos>,
    mut cursor_moved_events: EventReader<CursorMoved>,
){
    let mut should_update = false;
    for _event in cursor_moved_events.iter() {
        should_update = true;
        break;
    }

    if !should_update { return };
    //println!("Update display action points");

    clear_action_points_cursor_ui(&mut commands, interface_query);

    let Ok(player) = player_q.get_single() else { return };
    //let ap_cost_result = get_ap_cost(query_character, query_occupied, board, cursor.grid_position, player);
 
    let mut ap_valid = false;
    let mut ap_result = format!("x");
    if let Some(ap_cost) = action_infos.cost {
        let ap_char = ap_cost.to_string(); 
        ap_valid = true;
        ap_result = ap_char;
    }

    let (camera, camera_transform) = camera_q.single();
    let Some(screen_size) = camera.logical_viewport_size() else { return };    // What we can see in the screen. Some(Vec2(1422.0, 800.0) So 0,1422 and 1422, 800.0 for each corner.
    //let Some(screen_position) = cursor.screen_position else { return };

    //println!("Camera physical viewport size is {:?}", screen_size);

    for transform in query_game_cursor.iter() {
        let Some(screen_position) = camera.world_to_viewport(camera_transform, transform.translation)  else { continue };
        //If not in screen, we don't display.
        if screen_position.x < 0.0 || screen_position.x > screen_size.x || screen_position.y < 0.0 || screen_position.y > screen_size.y { continue};
  
        let left = screen_position.x + (CHAR_SIZE as f32 / 2.0);
        let top = screen_position.y + (CHAR_SIZE as f32 / 2.0); 

        let width = CHAR_SIZE as f32; 
        let height = CHAR_SIZE as f32 / 2.0;

        let grow = CHAR_SIZE as f32 * 2.0;

        let ap_container = commands.spawn(NodeBundle {
            style: Style {                
                left: Val::Px(left),
                //right: Val::Px(right),
                top: Val::Px(top),
                //bottom: Val::Px(bottom),
                width: Val::Px(width),
                height: Val::Px(height),
                flex_grow: grow,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            //background_color: Color::rgba(0.0, 0.0, 1.0, 0.5 ).into(),
            ..default()
        }).id();  

        let mut ap_color = Color::RED;
        if ap_valid {
            ap_color = Color::YELLOW;
        };

        let cursor_action_display = commands.spawn(
            TextBundle::from_section(
                format!("{}", ap_result),     //("{}",action_points),
                TextStyle { 
                    font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
                    font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                    color: ap_color,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(8.)),            
                ..default()
            }),
        ).id();


        commands.entity(ap_container).insert(UiActionPointsOnCursor);
        commands.entity(ap_container).add_child(cursor_action_display);
    }

}




pub fn draw_interface(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interface_query: Query<Entity, With<InterfaceGame>>,
    player_info_query: Query<(Entity, &Name, &Health), With<Player>>,
    player_actions_query: Query<(Entity, &ActionPoints), With<Player>>,
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
    }).insert(InterfaceGame).id();  

    let player_action_display = commands.spawn(
        TextBundle::from_section(
            format!("{}",action_points),
            TextStyle { 
                font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
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
    
    commands.entity(container).add_child(player_action_display);
    commands.entity(container).add_child(player_name_tag);
    commands.entity(container).add_child(chunk_container);
    
  
}


pub fn draw_enemy_health(
    mut commands: Commands,
    enemies_q: Query<(&Health, &Transform), With<Monster>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,    
    interface_query: Query<Entity, With<UiEnemyHp>>,
){
    clear_enemy_hp_ui(&mut commands, interface_query);

    let (camera, camera_transform) = camera_q.single();
    let Some(screen_size) = camera.logical_viewport_size() else { return };    // What we can see in the screen. Some(Vec2(1422.0, 800.0) So 0,1422 and 1422, 800.0 for each corner.

    //println!("Camera physical viewport size is {:?}", screen_size);

    for (health, transform) in enemies_q.iter() {
        let Some(screen_position) = camera.world_to_viewport(camera_transform, transform.translation)  else { continue };
        //If not in screen, we don't display.
        if screen_position.x < 0.0 || screen_position.x > screen_size.x || screen_position.y < 0.0 || screen_position.y > screen_size.y { continue};
      
        let left =screen_position.x - (TILE_WIDTH_HALF as f32);
        //let right =screen_size.x - screen_position.x;
        let top =screen_position.y - (TILE_HEIGHT_HALF as f32); // REMEMBER : world = y goes from bottom to top (++)
        //let bottom = screen_size.y - screen_position.y;
        let width = (health.max as f32 * INTERFACE_HP_CHUNK_WIDTH) / 2.0; //INTERFACE_HP_CHUNK_WIDTH * (health.max as f32) / 2.0;
        let height = INTERFACE_HP_CHUNK_HEIGHT/ 2.0;
        //println!("Character screen position is : {:?}", screen_position);
        //println!("left : {:?}, right : {:?}, top : {:?}, bottom : {:?}, width: {:?}, height: {:?}", left ,right ,top ,bottom, width, height );


        let grow = (health.max as f32 * INTERFACE_HP_CHUNK_WIDTH) / 2.0;

        let chunk_container = commands.spawn(NodeBundle {
            style: Style {                
                left: Val::Px(left),
                //right: Val::Px(right),
                top: Val::Px(top),
                //bottom: Val::Px(bottom),
                width: Val::Px(width),
                height: Val::Px(height),
                flex_grow: grow,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            //background_color: Color::rgba(0.0, 0.0, 1.0, 0.5 ).into(),
            ..default()
        }).id();  
        commands.entity(chunk_container).insert(UiEnemyHp);

        let mut chunk_list:Vec<Entity> = Vec::new();
        for i in 1..=health.max {
            let mut border_color = Color::rgb(0.5, 0.0, 0.0);
            let mut background_color = Color::rgb(0.9, 0.0, 0.0 );
            if i > health.current {
                border_color = Color::rgb(0.1, 0.1, 0.1);
                background_color = Color::rgba(0.0, 0.0, 0.0, 1.0 );
            }
    
            let chunk = commands.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(INTERFACE_HP_CHUNK_WIDTH / 2.0),//(8.0),
                    height: Val::Px(INTERFACE_HP_CHUNK_HEIGHT/ 2.0), //(16.0),
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


    }

}