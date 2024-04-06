use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::ui::{ui_game_logs::draw_log_ui, ReloadUiEvent};

pub struct GameLogsPlugin;

impl Plugin for GameLogsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Gamelog { entries: Vec::new()})

            .add_event::<LogEvent>()

            .add_systems(Update, log_received.run_if(on_event::<LogEvent>()))
            .add_systems(Update, draw_log_ui.run_if(on_event::<ReloadUiEvent>()))
            ;
    }
}

#[derive(Resource, Clone, Default, Deserialize, Serialize, Debug)]  
pub struct Gamelog {
    pub entries : Vec<String>
}
impl Gamelog {
    pub fn get_last_entries_as_string(&self, number:usize
    ) -> String {
        let mut logs = "".to_string();
        for log in self.entries.iter().rev().take(number).rev() {
            logs = format!("{}{}\n", logs, log.clone());
            //println!("LOG:Added to Log: {}", log.clone());
        }
        logs
    }
    pub fn clear(&mut self) {
        self.entries = Vec::new();
    }
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
        event_nb += 1;
    }    
    if event_nb > 0 {
        ev_ui.send(ReloadUiEvent);
    }
}