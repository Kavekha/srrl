use bevy::prelude::*;

use crate::{
    engine::{asset_loaders::GraphicsAssets, render::components::GameCursorRender},
    game::{
        combat::action_infos::ActionInfos, player::Player
    }, 
    globals::{CHAR_SIZE, INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE}
};

use super::{components::UiActionPointsOnCursor, ReloadUiEvent};


//===
const UI_CURSOR_DISPLAY_AP_VALID:Color = Color::YELLOW;
const UI_CURSOR_DISPLAY_AP_NOTVALID:Color = Color::RED;
//===

pub fn update_ui_game_cursor_from_action(
    mut ev_refresh_ui: EventReader<ReloadUiEvent>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    entity_cursor_q: Query<Entity, With<GameCursorRender>>,
    action_infos: Res<ActionInfos>,
    mut cursor_q: Query<&mut Handle<Image>>,
    graph_assets: Res<GraphicsAssets>
){
     // On peut être rafraichi de deux facons: Mouvement Mouse, ou Request de refresh.
    let mut should_update = false;
    for _event in cursor_moved_events.read() {
        should_update = true;
        break;
    }
    for _event in ev_refresh_ui.read() {
        should_update = true;
        break;
    }
    if !should_update { return };
    println!("Je dois mettre à jour l'apparence du Curseur.");   

    // Transformation du curseur
    let Ok(entity) = entity_cursor_q.get_single() else { return };

    if let Ok(mut cursor) = cursor_q.get_mut(entity) {
        if action_infos.target.is_some() {
            *cursor = graph_assets.cursors["cursor_targeting"].clone();
        } else {
            *cursor = graph_assets.cursors["cursor_moving"].clone();
        }        
    };

    
}

pub fn update_ui_game_cursor_display_action_points(
    mut ev_refresh_ui: EventReader<ReloadUiEvent>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut ap_cursor_q: Query<&mut Text, With<UiActionPointsOnCursor>>,
    player_q: Query<Entity, With<Player>>,
    action_infos: Res<ActionInfos>,
){
    // On peut être rafraichi de deux facons: Mouvement Mouse, ou Request de refresh.
    let mut should_update = false;
    for _event in cursor_moved_events.read() {
        should_update = true;
        break;
    }
    for _event in ev_refresh_ui.read() {
        should_update = true;
        break;
    }
    if !should_update { return };
    println!("Je dois mettre à jour l'affichage des AP à coté du Curseur.");

    let Ok(_player) = player_q.get_single() else { return };
    //let ap_cost_result = get_ap_cost(query_character, query_occupied, board, cursor.grid_position, player);
    
    let mut ap_valid = false;
    let mut ap_result = format!("x");
    if let Some(ap_cost) = action_infos.cost {
        let ap_char = ap_cost.to_string(); 
        ap_valid = true;
        ap_result = ap_char;
    }

    for mut text in &mut ap_cursor_q {
        println!("J'ai un Text pour UiActionPointsOnCursor");
        text.sections[0].value = format!("{ap_result}");
        text.sections[0].style.color = UI_CURSOR_DISPLAY_AP_NOTVALID.into();
        if ap_valid {
            text.sections[0].style.color = UI_CURSOR_DISPLAY_AP_VALID.into();
        };
    }
}


//https://bevyengine.org/examples/UI%20(User%20Interface)/viewport-debug/
pub fn update_ui_game_cursor_position_action_points(
    mut ev_refresh_ui: EventReader<ReloadUiEvent>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut ap_cursor_style_q: Query<&mut Style, With<UiActionPointsOnCursor>>,
    camera_q: Query<(&Camera, &GlobalTransform)>, 
    query_game_cursor: Query<&mut Transform, With<GameCursorRender>>,
){
    // On peut être rafraichi de deux facons: Mouvement Mouse, ou Request de refresh.
    let mut should_update = false;
    for _event in cursor_moved_events.read() {
        should_update = true;
        break;
    }
    for _event in ev_refresh_ui.read() {
        should_update = true;
        break;
    }
    if !should_update { return };
    println!("Je dois mettre à jour la position des AP à coté du curseur");


    for mut style in &mut ap_cursor_style_q {
        println!("J'ai un Style pour positionner mes AP à coté du curseur");
        let (camera, camera_transform) = camera_q.single();
        let Some(screen_size) = camera.logical_viewport_size() else { return };    // What we can see in the screen. Some(Vec2(1422.0, 800.0) So 0,1422 and 1422, 800.0 for each corner.

        for transform in query_game_cursor.iter() {
            println!("J'ai un Transform de ma Query Game Cursor");

            //==== On calcule à partir de la grille IG / Camera2d où placer l'UI.
            // TODO : Rassembler ce calcul avec celui utilisé dans ui_game_npc_infos pour afficher les HP enemis?
            let Some(screen_position) = camera.world_to_viewport(camera_transform, transform.translation)  else { continue };
            //If not in screen, we don't display.
            if screen_position.x < 0.0 || screen_position.x > screen_size.x || screen_position.y < 0.0 || screen_position.y > screen_size.y { continue};
    
            let left = screen_position.x + (CHAR_SIZE as f32 / 2.0);
            let top = screen_position.y + (CHAR_SIZE as f32 / 2.0); 

            let width = CHAR_SIZE as f32; 
            let height = CHAR_SIZE as f32 / 2.0;

            let grow = CHAR_SIZE as f32 * 2.0;

            //println!("Before: {:?}, {:?}, {:?}, {:?}, {:?}", style.left, style.top, style.width, style.height, style.flex_grow);

            style.left = Val::Px(left);
            //right: Val::Px(right),
            style.top = Val::Px(top);
            //bottom: Val::Px(bottom),
            style.width = Val::Px(width);
            style.height = Val::Px(height);
            style.flex_grow = grow;

            //println!("After: {:?}, {:?}, {:?}, {:?}, {:?}", style.left, style.top, style.width, style.height, style.flex_grow);
        }
    }
}

// Refacto 0.19g
pub fn draw_ui_cursor_action_points(
    mut commands: Commands,
    assets: Res<GraphicsAssets>
){
    println!("Draw UI Cursor action points : in progress");
    // Le Container. On ne va pas l'attacher à la Main Window pour pas foutre le dawa, et car independant de l'interface.
    let ap_container = commands.spawn(NodeBundle {
        style: Style {                
            //left: Val::Px(left),
            //right: Val::Px(right),
            //top: Val::Px(top),
            //bottom: Val::Px(bottom),
            //width: Val::Px(width),
            //height: Val::Px(height),
            //flex_grow: grow,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            position_type: PositionType::Absolute,
            ..default()
        },
        //background_color: Color::rgba(0.0, 0.0, 1.0, 0.5 ).into(),
        ..default()
    })//.insert(UiActionPointsOnCursor).insert(UiGameInterface)
    .id();  
    
    let cursor_action_display = commands.spawn(
        TextBundle::from_section(
            format!(""), 
            TextStyle { 
                font: assets.font.clone(),  
                font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                //color: ap_color,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(8.)),            
            ..default()
        }),
    ).insert(UiActionPointsOnCursor).id();

    commands.entity(ap_container).add_child(cursor_action_display);
    println!("Draw UI Cursor action points : iteration: OK");
}

