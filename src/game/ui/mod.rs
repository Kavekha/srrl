use bevy::prelude::*;

pub mod game_interface;


use crate::states::EngineState;

use self::game_interface::draw_interface;


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_systems(OnEnter(EngineState::PlayerInput), player_input_start)
            .add_systems(Update, draw_interface.run_if(on_event::<ReloadUiEvent>())
            );
    }
}

#[derive(Event)]
pub struct ReloadUiEvent;

fn player_input_start(
    mut ev_ui: EventWriter<ReloadUiEvent>
) {
    ev_ui.send(ReloadUiEvent);
}


