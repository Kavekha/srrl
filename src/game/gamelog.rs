use bevy::prelude::*;

use super::ui::{game_interface::display_log_ui, ReloadUiEvent};

pub struct GameLogsPlugin;

impl Plugin for GameLogsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Gamelog { entries: Vec::new()})

            .add_event::<LogEvent>()
            
            .add_systems(Update, log_received.run_if(on_event::<LogEvent>()))
            //.add_systems(Update, display_log_ui.run_if(on_event::<ReloadUiEvent>()))
            .add_systems(Update, display_log_ui.run_if(on_event::<ReloadUiEvent>()))
            ;
    }
}

#[derive(Resource)]
pub struct Gamelog {
    pub entries : Vec<String>
}

#[derive(Event)]
pub struct LogEvent{pub entry: String}

fn log_received(
    mut ev_log: EventReader<LogEvent>,
    mut gamelog: ResMut<Gamelog>,
    mut ev_ui: EventWriter<ReloadUiEvent>
){
    let mut event_nb = 0;
    for event in ev_log.read() {
        gamelog.entries.push(event.entry.to_string());
        println!("LOG: {}", event.entry);
        event_nb += 1;
    }    
    if event_nb > 0 {
        ev_ui.send(ReloadUiEvent);
    }
}