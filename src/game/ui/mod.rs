use bevy::prelude::*;

pub mod game_interface;


use crate::{states::{EngineState, GameState}, despawn_screen};

use self::game_interface::{draw_interface, InterfaceGame};

use super::actions::ActionExecutedEvent;


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_systems(OnEnter(EngineState::PlayerInput), turn_update_end)
            .add_systems(Update, turn_update_end.run_if(on_event::<ActionExecutedEvent>()).run_if(in_state(GameState::GameMap)))
            .add_systems(Update, draw_interface.run_if(on_event::<ReloadUiEvent>()).run_if(in_state(GameState::GameMap)))
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<InterfaceGame>)
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


