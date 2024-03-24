use bevy::prelude::*;

pub mod game_interface;
mod components;


use crate::game::states::GameState;

use self::{game_interface::{draw_interface, draw_enemy_health, display_action_points_on_cursor}, components::{InterfaceGame, UiEnemyHp, UiActionPointsOnCursor}};

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
            

            .add_systems(OnEnter(GameState::Disabled), despawn_component::<InterfaceGame>)
            .add_systems(OnEnter(GameState::Disabled), despawn_component::<UiEnemyHp>)
            .add_systems(OnEnter(GameState::Disabled), despawn_component::<UiActionPointsOnCursor>)
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


