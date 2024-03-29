// === DOCUMENTATION
/*
Les UI en jeu utilisent le tag UiGameInterface sur le container, pour pouvoir être supprimé quand on disabled la partie en cours.
Un tag UI commence par Ui dans l'ideal.

 */

use bevy::prelude::*;

pub mod game_interface;
mod components;


use crate::game::states::GameState;

use self::{components::{UiGameInterface, UiActionPointsOnCursor, UiEnemyHp, UiLog}, game_interface::{display_action_points_on_cursor, draw_enemy_health, draw_interface}};

use super::{despawn_component, combat::{CombatSet, event_systems::create_action_infos}};


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ReloadUiEvent>()

            .add_systems(OnEnter(GameState::Initialise), display_interface)

            .add_systems(Update, draw_interface.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::Running)))
            .add_systems(Update, draw_enemy_health.run_if(in_state(GameState::Running)))
            .add_systems(Update, display_action_points_on_cursor.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(create_action_infos))

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


// A cause de command / mut commands, on ne peut utiliser que celle-ci en systeme.
pub fn clear_all_game_interface(    
    interface_query: Query<Entity, With<UiGameInterface>>,
    mut commands: Commands,
) {
    despawn_component(interface_query, &mut commands);
}

// A cause de command / mut command, on ne peut utiliser que celle-ci au sein d'un systeme.... TODO?
pub fn clear_ui_game_interface(
    interface_query: Query<Entity, With<UiGameInterface>>,
    commands: &mut Commands,
) {
    despawn_component(interface_query, commands);
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



