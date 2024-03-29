use bevy::prelude::*;

use crate::{
    engine::{asset_loaders::GraphicsAssets, render::components::GameCursorRender},
    game::{
        combat::{event_systems::ActionInfos, events::RefreshActionCostEvent}, despawn_component, player::Player
    }, 
    globals::{CHAR_SIZE, INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE}
};

use super::components::{UiActionPointsOnCursor, UiGameInterface};



pub fn clear_ui_action_points_cursor(
    commands: &mut Commands,    
    interface_query: Query<Entity, With<UiActionPointsOnCursor>>,
) {
    //println!("DEBUG: Clear action points on Cursor ui.");
    despawn_component(interface_query, commands);
}


pub fn draw_ui_action_points_cursor(
    mut commands: Commands,
    assets: Res<GraphicsAssets>,
    camera_q: Query<(&Camera, &GlobalTransform)>, 
    query_game_cursor: Query<&mut Transform, With<GameCursorRender>>,
    interface_query: Query<Entity, With<UiActionPointsOnCursor>>,
    player_q: Query<Entity, With<Player>>,
    action_infos: Res<ActionInfos>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut ev_refresh_ap: EventReader<RefreshActionCostEvent>,

){
    let mut should_update = false;
    for _event in cursor_moved_events.read() {
        should_update = true;
        break;
    }
    for _event in ev_refresh_ap.read() {
        should_update = true;
        break;
    }

    if !should_update { return };
    //println!("Update display action points");

    clear_ui_action_points_cursor(&mut commands, interface_query);

    let Ok(_player) = player_q.get_single() else { return };
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
                    font: assets.font.clone(),  
                    //font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
                    font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                    color: ap_color,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(8.)),            
                ..default()
            }),
        ).id();


        commands.entity(ap_container).insert(UiActionPointsOnCursor).insert(UiGameInterface);   // TODO : la presence du UiGameInterface efface le Cursor et ne rafraichi le nombre qu'au deplacement souris. 
        commands.entity(ap_container).add_child(cursor_action_display);
    }

}
