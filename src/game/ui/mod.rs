// === DOCUMENTATION
/*
Les UI en jeu utilisent le tag UiGameInterface sur le container, pour pouvoir être supprimé quand on disabled la partie en cours.
Un tag UI commence par Ui dans l'ideal.

 */

 // !!! TO TEST : L'UI passe son temps à être Update sans recevoir d'event pour cela. Ca consomme! A verifier.

use bevy::prelude::*;

pub mod ui_game_interface;
pub mod ui_game_cursor;
pub mod ui_game_logs;
pub mod ui_game_npc_infos;
pub mod ui_game_attacks;
mod components;


use crate::game::states::GameState;

use self::{components::{ UiGameInterface, UiMainWindow}, ui_game_attacks::draw_ui_game_attack_icons, ui_game_cursor::draw_ui_action_points_cursor, ui_game_interface::{draw_ui_game_character_infos, update_ui_character_action_points, update_ui_character_health}, ui_game_npc_infos::draw_ui_game_enemy_hp};

use super::{despawn_component, combat::{CombatSet, action_infos::update_action_infos}};


pub const INTERFACE_HP_CHUNK_HEIGHT: f32 = 16.;
pub const INTERFACE_HP_CHUNK_WIDTH: f32 = 8.;
pub const INTERFACE_HP_CHUNK_MAX: u32 = 20;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ReloadUiEvent>()

            .add_systems(OnEnter(GameState::Initialise), display_interface)
            .add_systems(OnEnter(GameState::Initialise), draw_ui_main_window)

            // Refacto 0.19f : Nouveau fonctionnement UI.
            .add_systems(OnEnter(GameState::Initialise), draw_ui_game_character_infos.after(draw_ui_main_window))  // On lance dés le debut.
            .add_systems(Update, update_ui_character_health.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::Running)))
            .add_systems(Update, update_ui_character_action_points.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::Running)))

            

            //.add_systems(Update, draw_ui_game_character_infos.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::Running)))
            .add_systems(Update, draw_ui_game_enemy_hp.run_if(in_state(GameState::Running)))
            .add_systems(Update, draw_ui_action_points_cursor.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(update_action_infos))
            //.add_systems(Update, draw_ui_game_attack_icons.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::Running)))            
            .add_systems(OnEnter(GameState::Disabled), clear_all_game_interface)
            ;
    }
}

#[derive(Event)]
pub struct ReloadUiEvent;


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
            //row_gap: Val::Px(text_style.font_size * 2.),    // ?
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).insert(UiGameInterface).insert(UiMainWindow);  
}




// A cause de command / mut commands, on ne peut utiliser que celle-ci en systeme.
pub fn clear_all_game_interface(    
    interface_query: Query<Entity, With<UiGameInterface>>,
    mut commands: Commands,
) {
    println!("DEBUG: Leaving Game: Clear interface.");
    despawn_component(interface_query, &mut commands);
}








