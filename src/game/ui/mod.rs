use bevy::prelude::*;

pub mod game_interface;


use crate::{states::{EngineState, GameState}};

use self::game_interface::{draw_interface, InterfaceGame, draw_enemy_health, UiEnemyHp};

use super::{actions::ActionExecutedEvent, despawn_component};


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_systems(OnEnter(EngineState::PlayerInput), turn_update_end)
            .add_systems(Update, turn_update_end.run_if(on_event::<ActionExecutedEvent>()).run_if(in_state(GameState::GameMap)))
            .add_systems(Update, draw_interface.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::GameMap)))

            //.add_systems(Update, draw_enemy_health.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::GameMap)))
            .add_systems(Update, draw_enemy_health.run_if(in_state(GameState::GameMap)))
            .add_systems(OnExit(GameState::GameMap), despawn_component::<InterfaceGame>)
            .add_systems(OnExit(GameState::GameMap), despawn_component::<UiEnemyHp>)
            ;
    }
}


#[derive(Event)]
pub struct ReloadUiEvent;

fn turn_update_end(
    mut ev_ui: EventWriter<ReloadUiEvent>
) {
    ev_ui.send(ReloadUiEvent);
}


