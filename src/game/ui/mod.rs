// === DOCUMENTATION
/*
A LIRE ICI : https://cssreference.io/flexbox/#align-self

draw_ui_main_window : 
    En Row. Chaque élement ajouté s'ajoutera sur la même ligne.
    AlignItems:Flexstar les fait commencer de la gauche.
draw_ui_game_character_infos:
    En Column, mais de façon à ce que ca s'ecrase pour 

 */

 /* 0.19f : 
    1. Main Window + Game Char Infos : OK 
        MainWindow:
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,       
            flex_direction: FlexDirection::Row,
            bottom: Val::Px(0.),
        GameCharInfos: 
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            align_content: AlignContent::FlexEnd,   
            justify_content: JustifyContent::FlexStart, 
            align_items: AlignItems::FlexEnd,
            flex_direction: FlexDirection::Row,            
            bottom: Val::Px(10.),
    2. Step1 + GameAttack: OK => Attack a la suite de GameCharInfos (icone à 32 px)
        GameAttack:
            position_type: PositionType::Relative,
            align_content: AlignContent::Center,
            align_items: AlignItems::FlexEnd, 
            justify_content: JustifyContent::FlexEnd, 
            flex_direction: FlexDirection::Row,
            width: Val::Percent(50.),
            height: Val::Percent(20.),
            bottom: Val::Px(10.),
            ..default()
        Historique::
            A.Change: Attack:position_type::Relative => No change.
            B.Change: GameChar:width: Val::Percent(40.0), => No change.
            C.Change: Attack:justify_content: JustifyContent::FlexEnd => Attack en second row, mais une ligne au dessus de GameChar. => ACCEPTABLE
            D.Change: Attack:align_items: AlignItems::FlexStart, => Idem, mais Attack encore plus en hauteur (2-3 lignes de plus cette fois) => NOK 
            E.Change: Attack:align_items: AlignItems::FlexEnd, => Desormais en bas, p-e un peu trop. => OK 
            F.Change: Attack:bottom: Val::Px(20.), => OK !


  */


use bevy::prelude::*;

pub mod ui_game_interface;
pub mod ui_game_cursor;
pub mod ui_game_logs;
pub mod ui_game_npc_infos;
pub mod ui_game_attacks;
pub mod components;


use crate::game::states::GameState;

use self::{
    components::{ UiGameInterface, UiMainWindow}, 
    ui_game_attacks::{draw_ui_game_attack_icons, update_ui_game_attack_icons},
    ui_game_cursor::{draw_ui_cursor_action_points, update_ui_game_cursor_display_action_points, update_ui_game_cursor_rendor_from_available_action, update_ui_game_cursor_position_action_points}, 
    ui_game_interface::{draw_ui_game_character_infos, update_ui_character_action_points, update_ui_character_health}, 
    ui_game_logs::{draw_log_ui, update_ui_new_lines, update_ui_remove_old_lines}, 
    ui_game_npc_infos::{draw_ui_game_enemy_hp}
};

use super::{despawn_component, gamelog::LogEvent};

// Font Size
const INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE: f32 = 16.0;
const UI_LOG_LINE_FONT_SIZE: f32 = 8.;

// Chunk size
const INTERFACE_HP_CHUNK_HEIGHT: f32 = 16.;
const INTERFACE_HP_CHUNK_WIDTH: f32 = 8.;
const INTERFACE_HP_CHUNK_MAX: u32 = 20;

// Logs
const UI_LOG_LINE_MAX_DURATION_TIME: f32 = 4.0;


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ReloadUiEvent>()

            .add_systems(OnEnter(GameState::Running), (display_interface, draw_ui_main_window).chain())


            // TODO : Doit pouvoir se melanger en une seule fonction avec le Nextstate...
            .add_systems(OnEnter(GameState::Initialise), windows_mouse_desactivate)
            .add_systems(OnEnter(GameState::Running), windows_mouse_desactivate)
            .add_systems(OnEnter(GameState::Unavailable), windows_mouse_activate)
            .add_systems(OnEnter(GameState::Disabled), windows_mouse_activate)

            // Character UI
            .add_systems(OnEnter(GameState::Running), draw_ui_game_character_infos.after(draw_ui_main_window))
            .add_systems(Update, update_ui_character_health.run_if(on_event::<ReloadUiEvent>()))
            .add_systems(Update, update_ui_character_action_points.run_if(on_event::<ReloadUiEvent>()))
            // Attacks
            .add_systems(OnEnter(GameState::Running), draw_ui_game_attack_icons.after(draw_ui_main_window))       
            .add_systems(Update, update_ui_game_attack_icons.run_if(on_event::<ReloadUiEvent>()))
            // Cursor UI
            .add_systems(OnEnter(GameState::Running), draw_ui_cursor_action_points) 
            .add_systems(Update, (
                update_ui_game_cursor_display_action_points,
                update_ui_game_cursor_position_action_points,
                update_ui_game_cursor_rendor_from_available_action
            ).run_if(on_event::<ReloadUiEvent>()))  
            .add_systems(Update, (
                update_ui_game_cursor_display_action_points,
                update_ui_game_cursor_position_action_points,
                update_ui_game_cursor_rendor_from_available_action
            ).run_if(on_event::<CursorMoved>()))  
            // Draw Enemy HP
            .add_systems(Update, draw_ui_game_enemy_hp) // DEGUEU mais fait l'affaire. TODO Tooltip pour remplacer tout ca.
            // Log Ui                     
            .add_systems(OnEnter(GameState::Running), draw_log_ui) 
            .add_systems(Update, update_ui_new_lines.run_if(on_event::<LogEvent>()))
            .add_systems(Update, update_ui_remove_old_lines)   

            .add_systems(OnEnter(GameState::Disabled), clear_all_game_interface)
            .add_systems(OnEnter(GameState::Unavailable), clear_all_game_interface)
            ;
    }
}

#[derive(Event)]
pub struct ReloadUiEvent;

// TODO : Do better...
fn windows_mouse_desactivate(
    mut windows: Query<&mut Window>
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
}
fn windows_mouse_activate(
    mut windows: Query<&mut Window>
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = true;
}


fn display_interface(
    mut ev_ui: EventWriter<ReloadUiEvent>
) {
    ev_ui.send(ReloadUiEvent);
}

fn draw_ui_main_window(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,       
            flex_direction: FlexDirection::Row,
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).insert(UiGameInterface).insert(UiMainWindow);  
}


// A cause de command / mut commands, on ne peut utiliser que celle-ci en systeme.
fn clear_all_game_interface(    
    interface_query: Query<Entity, With<UiGameInterface>>,
    mut commands: Commands,
) {
    println!("DEBUG: Leaving Game: Clear interface.");
    despawn_component(interface_query, &mut commands);
}








