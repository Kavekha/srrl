use bevy::prelude::*;

use crate::{
    engine::{asset_loaders::GraphicsAssets, render::components::GameCursorRender},
    game::{
        combat::{action_infos::ActionInfos, combat_system::components::AttackType, events::Turn}, player::{Cursor, Player}, ui::{components::UiGameInterface, INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE}, visibility::components::View
    }, 
    globals::CHAR_SIZE
};

use super::{components::UiActionPointsOnCursor, ReloadUiEvent};
 

//===
const UI_CURSOR_DISPLAY_AP_VALID:Color = Color::YELLOW;
const UI_CURSOR_DISPLAY_AP_NOTVALID:Color = Color::RED;

pub const CURSOR_MOVING:&str = "cursor_moving";
const CURSOR_TARGETING:&str = "cursor_targeting";
const CURSOR_PUNCHING:&str = "cursor_punching";
const CURSOR_WAITING:&str = "cursor_waiting";
const CURSOR_CANT_SEE:&str = "cursor_cant_see";
//===


pub fn update_ui_game_cursor_rendor_from_available_action(
    mut ev_refresh_ui: EventReader<ReloadUiEvent>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    entity_cursor_q: Query<Entity, With<GameCursorRender>>,
    action_infos: Res<ActionInfos>,
    mut cursor_q: Query<&mut Handle<Image>>,
    graph_assets: Res<GraphicsAssets>,
    is_turn_q: Query<&Turn>,
    view_q: Query<&View>,
    res_cursor: Res<Cursor>,
){
     // On peut être rafraichi de deux facons: Mouvement Mouse, ou Request de refresh.
    let mut should_update = false;
    for _event in cursor_moved_events.read() {should_update = true; break;}
    for _event in ev_refresh_ui.read() { should_update = true; break;}
    if !should_update { return };

    //println!("Je dois mettre à jour l'apparence du Curseur.");  
    let Ok(entity) = entity_cursor_q.get_single() else { return };
    if let Ok(mut cursor) = cursor_q.get_mut(entity) {
        // Est-ce notre tour?
        let Some(player) = action_infos.entity else { return };
        // curseur dans une zone visible?   // 
        //REMEMBER : Ca sera chiant pour des actions que l'on pourrait faire à l'aveugle (jeter des grenades, etc). Ca s'applique à toutes les restrictions visuelles.
        if let Ok(view) = view_q.get(player) {
            if !view.visible_tiles.contains(&res_cursor.grid_position) {
                *cursor = graph_assets.cursors[CURSOR_CANT_SEE].clone(); 
                return;
            }
        }
        if let Ok(_turn) = is_turn_q.get(player) {
            // Our turn.
            if action_infos.attack == Some(AttackType::RANGED) { *cursor = graph_assets.cursors[CURSOR_TARGETING].clone();
            } else if action_infos.target.is_some() { *cursor = graph_assets.cursors[CURSOR_PUNCHING].clone();
            } else { *cursor = graph_assets.cursors[CURSOR_MOVING].clone(); }
        } else {
            // Not our turn.
            *cursor = graph_assets.cursors[CURSOR_WAITING].clone();
        }

    }    
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
    for _event in cursor_moved_events.read() { should_update = true; break; }
    for _event in ev_refresh_ui.read() { should_update = true; break; }
    if !should_update { return };
    //println!("Je dois mettre à jour l'affichage des AP à coté du Curseur.");
    let Ok(_player) = player_q.get_single() else { return };
    
    let mut ap_valid = false;
    let mut ap_result = format!("x");
    if let Some(ap_cost) = action_infos.cost {
        let ap_char = ap_cost.to_string(); 
        ap_valid = true;
        ap_result = ap_char;
    }

    for mut text in &mut ap_cursor_q {
        //println!("J'ai un Text pour UiActionPointsOnCursor");
        text.sections[0].value = format!("{ap_result}");
        text.sections[0].style.color = UI_CURSOR_DISPLAY_AP_NOTVALID.into();
        if ap_valid {
            text.sections[0].style.color = UI_CURSOR_DISPLAY_AP_VALID.into();
        };
    }
}


//https://bevyengine.org/examples/UI%20(User%20Interface)/viewport-debug/
//https://bevyengine.org/examples/2D%20Rendering/2d-viewport-to-world/
pub fn update_ui_game_cursor_position_action_points(
    mut ev_refresh_ui: EventReader<ReloadUiEvent>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut ap_cursor_style_q: Query<&mut Style, With<UiActionPointsOnCursor>>,
    windows: Query<&Window>,    // 0.20e
){
    // On peut être rafraichi de deux facons: Mouvement Mouse, ou Request de refresh.
    let mut should_update = false;
    for _event in cursor_moved_events.read() { should_update = true; break; }
    for _event in ev_refresh_ui.read() { should_update = true; break; }
    if !should_update { return };
    //println!("Je dois mettre à jour la position des AP à coté du curseur");

    for mut style in &mut ap_cursor_style_q {
        //let (camera, camera_transform) = camera_q.single();
        //let Some(screen_size) = camera.logical_viewport_size() else { return };    // What we can see in the screen. Some(Vec2(1422.0, 800.0) So 0,1422 and 1422, 800.0 for each corner.
        let Some(cursor_position) = windows.single().cursor_position() else {            
            return;
        };
        let left = cursor_position.x + (CHAR_SIZE as f32 / 2.0);
        let top = cursor_position.y + (CHAR_SIZE as f32 / 2.0); 
        let width = CHAR_SIZE as f32; 
        let height = CHAR_SIZE as f32 / 2.0;
        let grow = CHAR_SIZE as f32 * 2.0;

        style.left = Val::Px(left);
        style.top = Val::Px(top);
        style.width = Val::Px(width);
        style.height = Val::Px(height);
        style.flex_grow = grow;
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
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            position_type: PositionType::Absolute,
            ..default()
        },
        //background_color: Color::rgba(0.0, 0.0, 1.0, 0.5 ).into(),
        ..default()
    }).insert(UiGameInterface).id();  
    
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

